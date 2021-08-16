#![feature(proc_macro_hygiene, decl_macro)]

use redstone_rs::*;
use fern::colors::{Color, ColoredLevelConfig};
use log::*;
use redstone_rs::*;
use redstone_rs::block::{Block, Header};
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
use redstone_rs::rpc::{launch_client, Announcement, Caller};
use redstone_rs::keypair::Keypair;
use serde::{Deserialize, Serialize};
use lazy_static::*;
use std::{default::Default, sync::Mutex};

#[derive(Default)]
struct WalletDetails {
    wallet: Option<Keypair>,
    balance: u64,
    locked: u64,
    uncle_root: String,
}
lazy_static! {
    static ref WALLET_DETAILS: Mutex<WalletDetails> = Mutex::new(WalletDetails::default());
    static ref SERVER_ADDR: Mutex<String> = Mutex::new(String::from("http://127.0.0.1:8000"));
}
#[derive(Clone, Deserialize, Debug)]
struct Blockcount {
    success: bool,
    blockcount: u64,
}
#[derive(Clone, Deserialize, Debug)]
struct Transactioncount {
    success: bool,
    transaction_count: u64,
}

#[derive(Clone, Deserialize, Debug)]
struct HashAtHeight {
    success: bool,
    hash: String,
}



#[derive(Clone, Deserialize, Debug)]
struct Balances {
    success: bool,
    balance: u64,
    locked: u64,
}


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
            .level_for("launch_", log::LevelFilter::Off)
            .level_for("launch", log::LevelFilter::Off)
            .level_for("rocket::rocket", log::LevelFilter::Off)
            .level_for("api::start_api", log::LevelFilter::Info)
            .level_for("_", log::LevelFilter::Off)

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
       fs::write(&filename, encrypted).unwrap();
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
            reader.read_to_end(&mut decrypted).unwrap();
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
    println!("Private key:{}", wallet.private_key);
    info!("Enter Filename: ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename)
        .expect("Failed to read input.");
    info!("Enter Password: ");
    let mut pass = String::new();
    io::stdin().read_line(&mut pass)
        .expect("Failed to read input.");
    save_wallet(wallet.private_key,pass,filename.trim_end().to_string());
    info!("Wallet saved at: {}", filename);
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
    info!("[4] Show balance");
    info!("[8] Relogin");
    info!("[5] exit");
}


pub fn new_ann(ann: Announcement) {
    if let Ok(mut locked) = WALLET_DETAILS.lock() {
        debug!("Gained lock on WALLET_DETAILS");
        if ann.m_type == "block".to_string() {
            // ann.msg contains a block in json format
            if let Ok(blk) = serde_json::from_str::<Block>(&ann.content) {
                if  true {
                    let balance_before = locked.balance;
                    let locked_balance_before = locked.locked;
                    for txn in blk.transactions {
                        trace!("Txn: {:#?}", txn);
                        if txn.reciver == locked.wallet.as_ref().unwrap().public_key {
                            locked.balance += txn.amount;
                        }
                        if txn.sender == locked.wallet.as_ref().unwrap().public_key {
                            match txn.type_flag {
                                // 0 sended some rs to someone out balance - how much we sended
                                0 => {
                                    locked.balance -= txn.amount;
                                }
                                // 2 locked ballance for dpos?
                                8 => {
                                    if locked.balance > 64{
                                    locked.balance -= txn.amount;
                                    locked.locked += txn.amount;
                                    info!("Locked funds, commitment: {}", txn.hash);
                                    }else {
                                        info!("You need atleast {} to be validator. Your balance {}", 64 - locked.balance, locked.balance)
                                    
                                    }
                                }
                                _ => {
                                    error!(
                                        "Involved in unsupported transaction type, flag={}",
                                        txn.type_flag
                                    );
                                    debug!("Txn dump: {:#?}", txn);
                                }
                            }
                        }
                    }
                
                    if balance_before != locked.balance {
                        // Put it to the chain tx it on eg chain 1 top uncle roots push 1 
                        locked.uncle_root = blk.header.uncle_root.clone();
                        info!(
                            "New block {}, old balance: {}, new balance: {}",
                            blk.hash,
                            balance_before,
                            locked.balance
                        );
                        if locked_balance_before != locked.locked {
                            info!(
                                "Locked funds changed: old={} RS, new={} RS",
                                locked_balance_before, locked.locked
                            );
                        }
                    } else {
                        debug!("Block contained no transactions affecting us");
                    }
                } else {
                    debug!("Unkown");
                }
            }
        }
    }
}

fn main_login(pik: String,pbk: String){
    let wall = Keypair {
        private_key: pik.to_string(),
        public_key: pbk.to_string(),
    };
    if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
        *locked_ls = WalletDetails {
            wallet: Some(wall.clone()),
            balance: 0,
            locked: 0,
            uncle_root: "".to_string(),
        };
    }
    info!("Using wallet with publickey={}", pbk);
    debug!("Creating caller struct");
    let caller = Caller {
        callback: Box::new(new_ann),
    };
    thread::spawn(|| {
        launch_client("127.0.0.1".to_string(),44405,vec![],caller);
    });
    println!("Starting RPC client... at port:{}",44405);



    info!("Your wallet address:{}", pbk);
    println!("Private key:{}", pik);
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
        2 => {
            /*
            info!("Enter Redstone address: ");
            let mut address = String::new();
            io::stdin().read_line(&mut address)
                .expect("Failed to read input.");
            info!("Enter Redstone amount: ");
            let mut amount = String::new();
            io::stdin().read_line(&mut amount)
                .expect("Failed to read input.");
            info!("Enter Redstone password: ");
            let mut pass = String::new();
            io::stdin().read_line(&mut pass)
                .expect("Failed to read input.");
            */
            let sender = "";
            let receiver = "";
            let amount = 0;
            let typetx = 1;
            let payload = "";
            let send = redstone_rs::transaction::Transaction::new(sender.to_string(),receiver.to_string(),amount,typetx,payload.to_string());     
            println!("{:?}", send);
            info!("Sending...");
            main_login(pik,pbk);

        },
            
        5 => {
            info!("Bye....");

        }
        4 => {
            if let Ok(mut locked) = WALLET_DETAILS.lock() {
                debug!("Gained lock on WALLET_DETAILS");
                info!("RS balance: {}",locked.balance);
            }
            main_login(pik,pbk);

        }
        8 => { 
         main_login(pik,pbk);
            info!("relog");
            //dont exit loop back in here
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
        info!("Bye....");
        //save enverything

       }
       _ => {
           main_not_logged();
           println!("Unknown command");

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
