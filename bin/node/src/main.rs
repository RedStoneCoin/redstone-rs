#![feature(proc_macro_hygiene, decl_macro)]

use fern::colors::{Color, ColoredLevelConfig};
use log::*;
use redstone_rs::*;
mod api;
use std::collections::HashMap;
use redstone_rs::block::{Header,Block};
use redstone_rs::transaction::Transaction;
use redstone_rs::crypto::hash;
extern crate clap;
use clap::{Arg, App, SubCommand};
use redstone_rs::rpc::{block_announce, Announcement, Caller};
use std::{thread, time};

fn setup_logging(verbosity: u64) -> Result<(), fern::InitError> {
    let mut base_config = fern::Dispatch::new();
    base_config = match verbosity {
        0 => {
            // Let's say we depend on something which whose "info" level messages are too
            // verbose to include in end-user output. If we don't need them,
            // let's not include them.
            base_config
                .level(log::LevelFilter::Error)
                .level_for("redstone_rs", log::LevelFilter::Error)
                .level_for("node", log::LevelFilter::Error)
        }
        1 => base_config
            .level(log::LevelFilter::Warn)
            .level(log::LevelFilter::Error)
            .level_for("redstone_rs", log::LevelFilter::Warn)
            .level_for("node", log::LevelFilter::Warn),

        2 => base_config
            .level(log::LevelFilter::Warn)
            .level_for("redstone_rs", log::LevelFilter::Info)
            .level_for("node", log::LevelFilter::Info),

        3 => base_config
            .level(log::LevelFilter::Warn)
            .level(log::LevelFilter::Info)
            .level_for("redstone_rs", log::LevelFilter::Debug)
            .level_for("node", log::LevelFilter::Debug)
            .level_for("launch_", log::LevelFilter::Off)
            .level_for("launch", log::LevelFilter::Off)
            .level_for("rocket::rocket", log::LevelFilter::Off)
            .level_for("api::start_api", log::LevelFilter::Info)
            .level_for("_", log::LevelFilter::Off),

        _ => base_config
            .level(log::LevelFilter::Warn)
            .level_for("redstone_rs", log::LevelFilter::Trace)
            .level_for("node", log::LevelFilter::Trace),
    };

    // Separate file config so we can include year, month and day in file logs
    let file_config = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file("redstone-daemon.log")?);

    let stdout_config = fern::Dispatch::new()
        .format(|out, message, record| {
            let colors = ColoredLevelConfig::default()
                .info(Color::Green)
                .debug(Color::Magenta);
            // special format for debug messages coming from our own crate.
            if record.level() > log::LevelFilter::Info && record.target() == "cmd_program" {
                out.finish(format_args!(
                    "---\nDEBUG: {}: {}\n---",
                    chrono::Local::now().format("%H:%M:%S"),
                    message
                ))
            } else {
                out.finish(format_args!(
                    "{}[{}][{}] {}",
                    chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                    record.target(),
                    colors.color(record.level()),
                    message
                ))
            }
        })
        .chain(std::io::stdout());

    base_config
        .chain(file_config)
        .chain(stdout_config)
        .apply()?;
    std::panic::set_hook(Box::new(|pan| {
        error!("FATAL: {}", pan);
    }));
    Ok(())
}

fn cli() {
        /*
    let matches =  App::new("Redstone Node")
                        .version("0.1.0")
                        .author("Redstone Developers. <redstonecrypto@gmail.com>")
                        .about("Redstone Deamon Software")
                          .arg(Arg::with_name("validator")
                          .long("validator") // allow --name
                          .takes_value(true)
                          .help("validator")
                          .required(false))
                          .arg(Arg::with_name("mode")
                          .long("mode") // allow --name
                          .takes_value(true)
                          .help("mode")
                          .required(false))
                          .arg(Arg::with_name("logging")
                          .long("logging") // allow --name
                          .takes_value(true)
                          .help("logging level")
                          .required(false))
                          .get_matches();
    
    // return vec of args
    let mut args = Vec::new();
    args.push(matches.value_of("validator").unwrap_or("").to_string());
    args.push(matches.value_of("mode").unwrap_or("").to_string());
    args.push(matches.value_of("logging").unwrap_or("").to_string());
    // setup logging
    match args[2].as_ref() {
        "0" => {
            setup_logging(1).unwrap();
        }
        _ => {
            setup_logging(3).unwrap();
        }
    }
    */
}
fn main() {
    // TODO CLI!
    setup_logging(3).unwrap();

    let p2p_port = 44404;
    let rpc_port = p2p_port + 1;
    let assci_art = "
    ██████╗ ███████╗██████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗    ███╗   ██╗ ██████╗ ██████╗ ███████╗
    ██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝    ████╗  ██║██╔═══██╗██╔══██╗██╔════╝
    ██████╔╝█████╗  ██║  ██║███████╗   ██║   ██║   ██║██╔██╗ ██║█████╗      ██╔██╗ ██║██║   ██║██║  ██║█████╗  
    ██╔══██╗██╔══╝  ██║  ██║╚════██║   ██║   ██║   ██║██║╚██╗██║██╔══╝      ██║╚██╗██║██║   ██║██║  ██║██╔══╝  
    ██║  ██║███████╗██████╔╝███████║   ██║   ╚██████╔╝██║ ╚████║███████╗    ██║ ╚████║╚██████╔╝██████╔╝███████╗
    ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝    ╚═╝  ╚═══╝ ╚═════╝ ╚═════╝ ╚══════╝                                                                                                       
";
    info!("{}",assci_art);

    info!("Starting redstone node");
    warn!("Warning, this software is not stable");
    warn!("Run at your own risk!");
    mempool::Mempool::init(HashMap::new()).unwrap();
    info!("Launching API server at 0.0.0.0:8000");
    let _ = std::thread::spawn(move || {
        api::start_api();
    });
    info!("API server launched");
    let _ = std::thread::spawn(move || {
        redstone_rs::rpc::launch(rpc_port);
    });
    info!("RPC server launched");
    // loop so program does not end
    loop {}
    


}
