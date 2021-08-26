#![feature(proc_macro_hygiene, decl_macro)]

use fern::colors::{Color, ColoredLevelConfig};
use log::*;
use redstone_rs::*;
mod api;
use std::collections::HashMap;
use redstone_rs::block::{Header,Block};
use redstone_rs::transaction::Transaction;
use redstone_rs::crypto::hash;

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

fn startnnode() {
}
fn main() {
    setup_logging(3).unwrap();
    let p2p_port = 44404;
    let rpc_port = p2p_port + 1;

    let art = " 
    ██████╗ ███████╗██████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗
    ██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝
    ██████╔╝█████╗  ██║  ██║███████╗   ██║   ██║   ██║██╔██╗ ██║█████╗  
    ██╔══██╗██╔══╝  ██║  ██║╚════██║   ██║   ██║   ██║██║╚██╗██║██╔══╝  
    ██║  ██║███████╗██████╔╝███████║   ██║   ╚██████╔╝██║ ╚████║███████╗
    ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝
    ";
    info!("{}",art);
    info!("Starting redstone node");
    warn!("Warning, this software is not stable");
    warn!("Run at your own risk!");

    // init mempool
    mempool::Mempool::init(HashMap::new()).unwrap();
    // init p2p
    // let _ = std::thread::spawn(move || {
    //    redstone_rs::p2p::start_server(p2p_port);
    // });


    // init rpc
    info!("Launching API server at 0.0.0.0:8000");

    let _ = std::thread::spawn(move || {
        api::start_api();
    });
    info!("API server launched");
    let _ = std::thread::spawn(move || {
        let mut txn = Transaction {
            hash: "".to_owned(),
            sender: "coinbase".to_owned(),
            reciver: "0x1f7d366bce0b46d0487295ec9bfc194aab8ddb85".to_owned(),
            amount: 69,
            nonce: 1,
            type_flag: 0,
            payload: "".to_owned(), // Hex encoded payload
            pow: "".to_owned(),     // Spam protection PoW
            signature: "".to_owned(),
        };
        let mut blk = Block {
            hash: "".to_owned(),
            header: Header {
                height: 1,
                timestamp: 1,
                chain: 1,
                parent_hash: "".to_owned(),
                state_hash: "".to_owned(),
                uncle_root: "".to_owned(),
                proposer: "".to_owned(), // the publickey of the proposer
                transactions_merkle_root: "".to_owned(),
                header_payload: 0,
                proof: "".to_owned(),              // The vrf proof of the proposer as hex
                proposer_signature: "".to_owned(), // proposers signature
                validator_signatures: vec!("".to_owned()),
                vrf: "".to_owned(), // the hex encoded vrf proof used to sellect next rounds validating commitee and proposer
            },
            transactions: vec![txn],
        };
        //info!("wait 5 sec");
        //thread::sleep(time::Duration::from_secs(5));
        //info!("announe block test");
        //block_announce(blk).unwrap();
    });
    let _ = std::thread::spawn(move || {

        redstone_rs::rpc::launch(rpc_port);
    });
    while true {
        //
        
    }
    
    // init p2p


}
