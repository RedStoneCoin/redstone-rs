#![feature(proc_macro_hygiene, decl_macro)]

use redstone_rs::*;
use fern::colors::{Color, ColoredLevelConfig};
use log::*;
use redstone_rs::*;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::fs;
use std::io::{Write};
use secrecy::Secret;
use encryptfile as ef;
use std::thread;
mod api;




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
                .level_for("wallet", log::LevelFilter::Error)
        }
        1 => base_config
            .level(log::LevelFilter::Warn)
            .level(log::LevelFilter::Error)
            .level_for("redstone_rs", log::LevelFilter::Warn)
            .level_for("wallet", log::LevelFilter::Warn),

        2 => base_config
            .level(log::LevelFilter::Warn)
            .level_for("redstone_rs", log::LevelFilter::Info)
            .level_for("wallet", log::LevelFilter::Info),

        3 => base_config
            .level(log::LevelFilter::Warn)
            .level(log::LevelFilter::Info)

            .level_for("redstone_rs", log::LevelFilter::Debug)
            .level_for("wallet", log::LevelFilter::Debug),

        _ => base_config
            .level(log::LevelFilter::Warn)
            .level_for("redstone_rs", log::LevelFilter::Trace)
            .level_for("wallet", log::LevelFilter::Trace),
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
        .chain(fern::log_file("redstone-wallet.log")?);

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
fn save_wallet(wallet: String,pass: String,filename: String) {
       let encrypted = {
           let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass.to_owned()));
           let mut encrypted = vec![];
           let mut writer = encryptor.wrap_output(&mut encrypted).unwrap();
           writer.write_all(wallet.as_bytes()).unwrap();
           writer.finish().unwrap();
           encrypted
       };
       fs::write(&filename, encrypted);
       info!("WALLET SAVED AT: {}", filename);
}
fn open_wallet(pass: String, filename: String) {
    //decryptit
    //then read it
    let private_key = std::fs::read(filename.trim_end())
        .expect("Something went wrong reading the file");
    let decrypted = {
            let decryptor = match age::Decryptor::new(&private_key[..]).unwrap() {
                age::Decryptor::Passphrase(d) => d,
                _ => unreachable!(),
            };
            let mut decrypted = vec![];
            let mut reader = decryptor.decrypt(&Secret::new(pass.to_owned()), None).unwrap();
            reader.read_to_end(&mut decrypted);
            decrypted
    };
    let decrypted1 = String::from_utf8(decrypted);
    let wallet = redstone_rs::keypair::Keypair::from_private_key(decrypted1.unwrap());
    print!("Wallet imported successfully!\n");
    main_login(wallet.private_key.to_string(),wallet.address());
}
fn gen_keypair() {
    let wallet = redstone_rs::keypair::Keypair::generate();
    info!("Your wallet address:{}", wallet.address());
    info!("Private key:{}", wallet.private_key);
    info!("Enter Filename: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)
        .expect("Failed to read input.");
    info!("Enter Password: ");
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Failed to read input.");
    save_wallet(wallet.private_key,pass,filename.trim_end().to_string());
    main_not_logged();

}

fn commands(){
    info!("[1] Generate a new wallet");
    info!("[2] Import private key");
    info!("[3] Import wallet file");
    info!("[4] exit");
}

fn commands_logged(){
    info!("[1] Show wallet balance");
    info!("[2] Send Redstone");
    info!("[3] Show transaction history");
    info!("[4] Show transaction details");
    info!("[5] exit");
}

fn main_login(pik: String,pbk: String){
    info!("Your wallet address:{}", pbk);
    info!("Private key:{}", pik);
    commands_logged();
    let mut input = String::new();
    // Reads the input from STDIN and places it in the String named input.
    info!("Enter a value:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    // Convert to an i32.
    let input: i32 = input.trim().parse().unwrap();
    match input {
        1 => {
            info!("Commint soon!");
        },
        5 => {
            info!("Bye!");
        }
        _ => {
            main_login(pik,pbk);
            info!("Unknown command");
            //dont exit loop back in here
        }
    }
}
fn wallet_control(command: i32) {
    match command {
    1 => {
            gen_keypair();
    },
    2 => {
        info!("Enter private key: ");
        let mut private_key = String::new();
        io::stdin().read_line(&mut private_key)
            .expect("Failed to read input.");
        let wallet = redstone_rs::keypair::Keypair::from_private_key(private_key.trim_end().to_string());
        info!("{:?}", wallet);
        //save to the file
        info!("Enter wallet filename: ");

        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
            .expect("Failed to read input.");
        info!("Enter Password: ");

        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Failed to read input.");
    
        save_wallet(wallet.private_key.to_string(),pass,filename.trim_end().to_string());
    },
    3 => {
        let mut filename = String::new();
        io::stdin().read_line(&mut filename)
            .expect("Failed to read input.");
        info!("{}", filename);
        info!("Enter wallet password: ");
        let mut pass = String::new();
        io::stdin().read_line(&mut pass)
            .expect("Failed to read input.");
        //decryptit
        open_wallet(pass,filename);

    } 
    _ => {
        main_not_logged();
        info!("Unknown command");

    }
  }
}
 
fn command_control(command: i32) {
   match command {
       1 => {
            wallet_control(1);
         }
       2 => {
           info!("Import wallet");
           wallet_control(2);
       }
       3 => {
           info!("Import wallet file");
           wallet_control(3);

       }
       4 => {
        info!("Exited");
        //save enverything

       }
       _ => {
           main_not_logged();
           info!("Unknown command");

       }
   }
}

pub fn get_input_int() {
    let mut input = String::new();
    // Reads the input from STDIN and places it in the String named input.
    info!("Enter a value:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    // Convert to an i32.
    let input: i32 = input.trim().parse().unwrap();
    command_control(input);
}
pub fn get_input_wallet() {
    let mut input = String::new();
    // Reads the input from STDIN and places it in the String named input.
    info!("Enter a value:");
    io::stdin().read_line(&mut input)
        .expect("Failed to read input.");
    // Convert to an i32.
    let input: i32 = input.trim().parse().unwrap();
    wallet_control(input);
}

fn main_not_logged() {

    info!("Welcome Redstone Wallet!");
    info!("ALPHA 0.1-a1!");
    info!("TEST NET WALLET!");

    commands();
    get_input_int();
}
fn main() {
    setup_logging(3).unwrap();

    // hread::spawn(|| {
    //     api::start_api();
    // });
    //start logging


    let art = " 
    ██████╗ ███████╗██████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗
    ██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝
    ██████╔╝█████╗  ██║  ██║███████╗   ██║   ██║   ██║██╔██╗ ██║█████╗  
    ██╔══██╗██╔══╝  ██║  ██║╚════██║   ██║   ██║   ██║██║╚██╗██║██╔══╝  
    ██║  ██║███████╗██████╔╝███████║   ██║   ╚██████╔╝██║ ╚████║███████╗
    ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝
    ";
    println!("{}", art);

    main_not_logged()

}
