
#![feature(proc_macro_hygiene, decl_macro)]
use fern::colors::{Color, ColoredLevelConfig};
use lazy_static::*;
use log::*;
use std::env;
use serde_json::{Value};
use redstone_rs::block::{Block};
use redstone_rs::keypair::Keypair;
use redstone_rs::rpc::{launch_client, Announcement, Caller};
use redstone_rs::transaction::Transaction;
use redstone_rs::*;
use secrecy::Secret;
use serde::{Deserialize};
use std::fs;
use std::io;
use std::io::prelude::*;
use std::io::Read;
use std::io::Write;
use std::thread;
use std::{default::Default, sync::Mutex};
use std::time;
use reqwest::Client;
use tokio;
use std::str;
use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window};
use fltk::input::Input;
extern crate clipboard;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use fltk::valuator::ValueInput;
#[derive(Default)]
struct WalletDetails {
    wallet: Option<Keypair>,
    balance: u64,
    locked: u64,
    uncle_root: String,
}
use fltk::input::SecretInput;
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
            .level(log::LevelFilter::Debug)

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

async fn send_transaction(txn: Transaction) {
    if txn.signature == "" {
        //tx not signed
        println!("Transaction not signed");
    } else {
    let txn_json = serde_json::to_string(&txn).unwrap();
    let request_url = SERVER_ADDR.lock().unwrap().to_owned() + "/json_api/submit_txn";

    if let Ok(response) = Client::new().post(request_url).json(&txn_json).send().await {
        if let Ok(response_string) = response.text().await {
            if response_string.contains("404") {
                info!("Failed to submit txn, response={}", response_string);
            } else {
                info!("Submit response={}", response_string);
                if response_string.len() == 0 {
                    info!("Transaction Failure");
                }
            }
        }
    }
    }
}

async fn get_account(addr: String) -> String {
    //Using format! hre removes one unnecessary allocation
    
    let request_url = format!("{}/json_api/get_acc/{}",SERVER_ADDR.lock().unwrap().to_owned(), addr);
    let body = reqwest::get(request_url.clone()).await;
    
    match reqwest::get(request_url.clone()).await{
        Err(eer) => {
            return "".to_string()
        }
        Ok(resp) => return resp.text().await.unwrap()
    }
}


fn save_wallet(wallet: String, pass: String, filename: String) {
    let encrypted = {
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(pass.trim_end().to_owned()));
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

    let private_key =
        std::fs::read(filename.trim_end()).expect("Something went wrong reading the file");
    let decrypted = {
        let decryptor = match age::Decryptor::new(&private_key[..]).unwrap() {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };
        let mut decrypted = vec![];
        let mut reader = decryptor
            .decrypt(&Secret::new(pass.trim_end().to_owned()), None)
            .unwrap();
        reader.read_to_end(&mut decrypted).unwrap();
        decrypted
    };
    let decrypted1 = String::from_utf8(decrypted);
    let wallet = redstone_rs::keypair::Keypair::from_private_key(decrypted1.unwrap());
    print!("Wallet imported successfully!\n");
    let walelt1 = wallet.clone();
    main_login(wallet.private_key.to_string(), wallet.public_key,walelt1.address(), false);
}
fn open_wallet_gui(pass: String, filename: String) {
    let private_key =
        std::fs::read(filename.trim_end()).expect("Something went wrong reading the file");
    println!("{:#?}", pass);
    let decrypted = {
        let decryptor = match age::Decryptor::new(&private_key[..]).unwrap() {
            age::Decryptor::Passphrase(d) => d,
            _ => unreachable!(),
        };
        let mut decrypted = vec![];
        let mut reader = decryptor
            .decrypt(&Secret::new(pass.trim_end().to_owned()), None)
            .unwrap();
        reader.read_to_end(&mut decrypted).unwrap();
        decrypted
    };
    let decrypted1 = String::from_utf8(decrypted);
    let wallet = redstone_rs::keypair::Keypair::from_private_key(decrypted1.unwrap());
    print!("Wallet imported successfully!\n");
    let walelt1 = wallet.clone();
    main_login_gui(wallet.private_key.to_string(), wallet.public_key,walelt1.address());
}
fn gen_keypair() {
    let wallet = redstone_rs::keypair::Keypair::generate();
    info!("Your wallet address:{}", wallet.address());
    println!("Private key:{}", wallet.private_key);
    info!("Enter Filename: ");
    let mut filename = String::new();
    io::stdin()
        .read_line(&mut filename)
        .expect("Failed to read input.");
    info!("Enter Password: ");
    let mut pass = String::new();
    io::stdin()
        .read_line(&mut pass)
        .expect("Failed to read input.");
    save_wallet(wallet.private_key, pass, filename.trim_end().to_string());
    info!("Wallet saved at: {}", filename);
}
fn gen_keypair_gui(pass: String, filename: String) {

    let wallet = redstone_rs::keypair::Keypair::generate();
    save_wallet(wallet.clone().private_key,(&pass.clone().to_string().trim_end()).to_string(), filename.to_string());
    println!("Wallet address:{}", wallet.clone().address());



    println!("Wallet saved at: {:?}", filename);
}

fn commands() {
    info!("[1] Generate a new wallet");
    info!("[2] Import private key");
    info!("[3] Import wallet file");
    info!("[4] exit");
}

fn commands_logged() {
    info!("[1] Show wallet balance");
    info!("[2] Send Redstone");
    info!("[3] Send Custom Transaction");
    
    info!("[4] exit");
}

pub fn new_ann(ann: Announcement) {
    if let Ok(mut locked) = WALLET_DETAILS.lock() {
        debug!("Gained lock on WALLET_DETAILS");
        if ann.m_type == "block".to_string() {
            debug!("Recieved block ann");
            // ann.msg contains a block in json format

            
            
            if let Ok(blk) = serde_json::from_str::<Block>(&ann.content) {
                if true {
                    let keypair = Keypair {
                        private_key: "".to_string(),
                        public_key: locked.wallet.as_ref().unwrap().public_key.to_string(),
                    };
                    let balance_before = locked.balance;
                    let locked_balance_before = locked.locked;
                    for txn in blk.transactions {
                        if txn.reciver == keypair.address() {
                            match txn.type_flag {
                                // 0 u got some rs
                                0 => {
                                    info!("You got RS: {}", txn.amount);

                                    locked.balance += txn.amount;
                                }
                                // someone deleagated funds to you
                                4 => {
                                    locked.locked += txn.amount;
                                    info!("Locked funds, commitment: {}", txn.hash);
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
                        if txn.sender == locked.wallet.as_ref().unwrap().public_key {
                            match txn.type_flag {
                                // 0 sended some rs to someone out balance - how much we sended
                                0 => {
                                    locked.balance -= txn.amount;
                                }
                                // 2 locked ballance for dpos?
                                4 => {
                                    if locked.balance > 64 {
                                        locked.balance -= txn.amount;
                                        locked.locked += txn.amount;
                                        info!("Locked funds, commitment: {}", txn.hash);
                                    } else {
                                        info!(
                                            "You need at least 64 ({} more) to be validator. Your balance {}",
                                            64 - locked.balance,
                                            locked.balance
                                        )
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
                            blk.hash, balance_before, locked.balance
                        );
                        if locked_balance_before != locked.locked {
                            info!(
                                "Locked funds changed: old={} RS, new={} RS",
                                locked_balance_before, locked.locked
                            );
                        }
                        drop(locked);
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

fn gui_tips() {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Redstone GUI Wallet Logged Tips v0.1");
    let mut label = Frame::new(0, 0, 300, 150, "Welcome to the Redstone Wallet\n\n");
    let mut label1 = Frame::new(0, 0, 300, 250, "To see balance click re-balance!\n\n");
    wind.end();
    wind.show();
    app.run().unwrap();
}
fn priv_key_gui(pik: String) {
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 400, 300, "Close this windows, Redstone Wallet v0.1");
    let mut but4 = Button::new(0, 0, 400, 300, "Copy Private Key");
    but4.set_callback(move |_| {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        println!("{:?}", ctx.get_contents());
        ctx.set_contents(pik.to_owned()).unwrap();
    });
    wind.end();
    wind.show();
    app.run().unwrap();
}
fn main_login_gui(pik: String, pbk: String, addr11: String) {
    let app = app::App::default();

    app::background(223,223,223);

    let mut wind = Window::new(100, 100, 800, 600, "Redstone GUI Wallet Logged v0.1");
    let mut addr = Frame::new(0, 50, 550, 40, "Address:");
    let mut addr = Frame::new(0, 50, 1000, 40, "Addr");
    let mut gui_bal = Frame::new(0, 70, 800, 40, "Balance:");
    let mut gui_bal1 = Frame::new(0, 70, 1000, 40, "");
    let mut gui_bal3 = Frame::new(0, 70, 1000, 40, "");
    let addr_copy = addr11.clone();
    let mut gui_notification = Frame::new(0, 100, 800, 40, "");
    let mut gui_notification1 = Frame::new(0, 100, 800, 40, "");

    addr.set_label(&addr11.clone());
    let mut addr_send = Input::new(70, 50, 150, 40, "Send to");
    let mut amount = Input::new(70, 110, 50, 40, "Amount");
    let mut but = Button::new(70, 210, 100, 40, "Send");
    let mut but1 = Button::new(70, 310, 100, 40, "Copy address");
    let mut but2 = Button::new(70, 260, 100, 40, "Re Balance");
    let mut but4 = Button::new(70, 360, 100, 40, "Private Key");
    let mut but3 = Button::new(70, 410, 100, 40, "Tips");


    let mut not = 0;
    
    but2.set_callback(move |_| {
        if let Ok(walletdetails) = WALLET_DETAILS.lock() {
            gui_bal1.set_label(&format!("{}", walletdetails.balance));
            drop(walletdetails);
        }
        gui_notification.set_label(&format!("Balance updated {}", not));
        not += 1;
    });
    let mut pik1 = pik.clone();

    but4.set_callback( move |_| {
        priv_key_gui(pik1.clone());
    });
    but1.set_callback(move |_| {
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        println!("{:?}", ctx.get_contents());
        ctx.set_contents(addr_copy.to_owned()).unwrap();
    });

    but3.set_callback(move |_| {
        gui_tips();
    });
 
    wind.end();
    wind.show();
    let wall = Keypair {
        private_key: pik.to_string(),
        public_key: pbk.to_string(),
    };
    let caller = Caller {
        callback: Box::new(new_ann),
    };

    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
        if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
            *locked_ls = WalletDetails {
                wallet: Some(wall.clone()),
                balance: 0,
                locked: 0,
                uncle_root: "".to_string(),
            };
            drop(locked_ls)
        }
        let gacc = get_account(addr11.clone()).await;
        debug!("{}",gacc);
        if gacc.clone() == "" {
            if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
                *locked_ls = WalletDetails {
                    wallet: Some(wall.clone()),
                    balance: 0,
                    locked: 0,
                    uncle_root: "".to_string(),
                };
                drop(locked_ls)

            }
        } else {
            let v: Value = serde_json::from_str(&gacc).unwrap();
            let val = &v["Result"]["balance"];
            if &v["result"] != "failure" {

                if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
                    *locked_ls = WalletDetails {
                        wallet: Some(wall.clone()),
                        balance: val.as_u64().expect("not a valid u64"),
                        locked: 0,
                        uncle_root: "".to_string(),
                };
                drop(locked_ls)
            }

            }
        }

    });
    if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
        *locked_ls = WalletDetails {
            wallet: Some(wall.clone()),
            balance: 0,
            locked: 0,
            uncle_root: "".to_string(),
        };
        drop(locked_ls);

    }

    if let Ok(mut locked) = WALLET_DETAILS.lock() {
        let caller = Caller {
            callback: Box::new(new_ann),
        };
        thread::spawn(|| {
            launch_client("127.0.0.1".to_string(), 44405, vec![], caller);
        });
        drop(locked);
    }
    but.set_callback(move |_| {
        println!("Send");
        if let Ok(walletdetails) = WALLET_DETAILS.lock() {
            
            let mut txn1 = Transaction {
                hash: "".to_owned(),
                sender: walletdetails
                    .wallet
                    .as_ref()
                    .unwrap()
                    .public_key
                    .to_owned(),
                reciver: addr_send.value().to_owned(),
                amount: amount.value().parse::<u64>().unwrap().to_owned(),
                nonce: 0,
                type_flag: 0,
                payload: "".to_owned(), // Hex encoded payload
                pow: "".to_owned(), // Spam protection PoW
                signature: "".to_owned(),
            };                    //99999999999999999999
            let pow = txn1.find_pow();

            let sign = walletdetails.wallet.as_ref().unwrap().sign(txn1.hash.clone());

            txn1.signature = walletdetails.wallet.as_ref().unwrap().sign(txn1.hash.clone()).unwrap();

            println!("{:?}",txn1);

            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    
                    send_transaction(txn1).await;
                });
        }
    });
    // thread with while loop to update the wallet


    app.run().unwrap();
}









fn main_login(pik: String, pbk: String, addr: String, launched: bool) {
    let wall = Keypair {
        private_key: pik.to_string(),
        public_key: pbk.to_string(),
    };

    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
        if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
            *locked_ls = WalletDetails {
                wallet: Some(wall.clone()),
                balance: 0,
                locked: 0,
                uncle_root: "".to_string(),
            };
            drop(locked_ls)
        }
        let gacc = get_account(addr.clone()).await;
        debug!("{}",gacc);
        if gacc.clone() == "" {
            if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
                *locked_ls = WalletDetails {
                    wallet: Some(wall.clone()),
                    balance: 0,
                    locked: 0,
                    uncle_root: "".to_string(),
                };
                drop(locked_ls)

            }
        } else {
            let v: Value = serde_json::from_str(&gacc).unwrap();
            let val = &v["Result"]["balance"];
            if &v["result"] != "failure" {
                if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
                    *locked_ls = WalletDetails {
                        wallet: Some(wall.clone()),
                        balance: val.as_u64().expect("not a valid u64"),
                        locked: 0,
                        uncle_root: "".to_string(),
                };
                drop(locked_ls)

            }

            }
        }

    });
    if let Ok(mut locked_ls) = WALLET_DETAILS.lock() {
        *locked_ls = WalletDetails {
            wallet: Some(wall.clone()),
            balance: 0,
            locked: 0,
            uncle_root: "".to_string(),
        };
    }
    if let Ok(mut locked) = WALLET_DETAILS.lock() {
        info!("Gained lock on WALLET_DETAILS");
        info!("Using wallet with publickey={}", pbk);
        info!("Creating caller struct");
        let caller = Caller {
            callback: Box::new(new_ann),
        };
        thread::spawn(|| {
            launch_client("127.0.0.1".to_string(), 44405, vec![], caller);
        });

        drop(locked);
        info!("Your wallet address:{}", addr);
        info!("Your wallet public key:{}", pbk);
        println!("Private key:{}", pik);
        thread::sleep(time::Duration::from_secs(2));
        commands_logged();
        while true {
            let mut input = String::new();
            // Reads the input from STDIN and places it in the String named input.
            info!("Enter a value:");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input.");
            // Convert to an i32.
            let input: u64 = input.trim().parse().unwrap();
            match input {
                    1 => {
                        if let Ok(walletdetails) = WALLET_DETAILS.lock() {
                            info!("Our balance: {}", walletdetails.balance);
                            drop(walletdetails);
                        }
                    }
                    2 => {
                        info!("Enter recivers pub key: ");
                        let mut reciver = String::new();
                        io::stdin()
                            .read_line(&mut reciver)
                            .expect("Failed to read input.");
                        let mut input = String::new();
                        info!("Enter a value:");
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input.");
                        let input: u64 = input.trim().parse().unwrap();

                        if let Ok(mut walletdetails) = WALLET_DETAILS.lock() {
                            if input < 1000 {
                                let mut txn1 = Transaction {
                                    hash: "".to_owned(),
                                    sender: walletdetails
                                        .wallet
                                        .as_ref()
                                        .unwrap()
                                        .public_key
                                        .to_owned(),
                                    reciver: reciver.trim_end().to_owned(),
                                    amount: input,
                                    nonce: 0,
                                    type_flag: 0,
                                    payload: "".to_owned(), // Hex encoded payload
                                    pow: "".to_owned(), // Spam protection PoW
                                    signature: "".to_owned(),
                                };                    //99999999999999999999
                                let pow = txn1.find_pow();
               
                                let sign = walletdetails.wallet.as_ref().unwrap().sign(txn1.hash.clone());

                                info!("hash for txn:{}", txn1.hash);
                                txn1.signature = walletdetails.wallet.as_ref().unwrap().sign(txn1.hash.clone()).unwrap();

                                println!("{:#?}", txn1);

                                tokio::runtime::Builder::new_multi_thread()
                                    .enable_all()
                                    .build()
                                    .unwrap()
                                    .block_on(async {
                                        
                                        send_transaction(txn1).await;
                                    });
                            } else {
                                info!(
                                    "You are tring to send more then you have!!! Balance: {}",
                                    walletdetails.balance
                                );
                            }
                            drop(walletdetails);
                        }
                    }
                    // send custom transaction where user will input everything in transaction
                    // format

                    3 => {
                        info!("Enter recivers pub key: ");
                        let mut reciver = String::new();
                        io::stdin()
                            .read_line(&mut reciver)
                            .expect("Failed to read input.");
                        let mut input = String::new();
                        info!("Enter a amout:");
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input.");
                        let input: u64 = input.trim().parse().unwrap();

                        let mut py = String::new();
                        info!("Enter payload:");
                        io::stdin()
                            .read_line(&mut py)
                            .expect("Failed to read input.");

                        let mut type_flag = String::new();
                        info!("Enter type_flag:");
                        io::stdin()
                            .read_line(&mut type_flag)
                            .expect("Failed to read input.");
                        let type_flag: u8 = type_flag.trim().parse().unwrap();

                        if let Ok(mut walletdetails) = WALLET_DETAILS.lock() {
                            if input < 1000 {
                                let mut txn1 = Transaction {
                                    hash: "".to_owned(),
                                    sender: walletdetails
                                        .wallet
                                        .as_ref()
                                        .unwrap()
                                        .public_key
                                        .to_owned(),
                                    reciver: reciver.trim_end().to_owned(),
                                    amount: input,
                                    nonce: 0,
                                    type_flag: type_flag,
                                    payload: py.to_owned(), // Hex encoded payload
                                    pow: "".to_owned(), // Spam protection PoW
                                    signature: "".to_owned(),
                                };                    //99999999999999999999
                                let pow = txn1.find_pow();
               
                                let sign = walletdetails.wallet.as_ref().unwrap().sign(txn1.hash.clone());

                                info!("hash for txn:{}", txn1.hash);
                                txn1.signature = walletdetails.wallet.as_ref().unwrap().sign(txn1.hash.clone()).unwrap();

                                println!("{:#?}", txn1);

                                tokio::runtime::Builder::new_multi_thread()
                                    .enable_all()
                                    .build()
                                    .unwrap()
                                    .block_on(async {
                                        send_transaction(txn1).await;
                                    });
                            } else {
                                info!(
                                    "You are tring to send more then you have!!! Balance: {}",
                                    walletdetails.balance
                                );
                            }
                            drop(walletdetails);
                        }
                    }
                    4 => {
                        info!("Bye....");
                        break;
                    }

                    8 => {
                        info!("relog");
                        // TODO: relog
                        break;
                    }
                    _ => {
                        info!("Unknown command");
                        //dont exit loop back in here
                    }
                }
        }
    }
}

fn wallet_control(command: i32) {
    match command {
        1 => {
            gen_keypair();
        }
        2 => {
            info!("Enter private key: ");
            let mut private_key = String::new();
            io::stdin()
                .read_line(&mut private_key)
                .expect("Failed to read input.");
            let wallet =
                redstone_rs::keypair::Keypair::from_private_key(private_key.trim_end().to_string());
            info!("{:?}", wallet);
            //save to the file
            info!("Enter wallet filename: ");

            let mut filename = String::new();
            io::stdin()
                .read_line(&mut filename)
                .expect("Failed to read input.");
            info!("Enter Password: ");

            let mut pass = String::new();
            io::stdin()
                .read_line(&mut pass)
                .expect("Failed to read input.");

            save_wallet(
                wallet.private_key.to_string(),
                pass,
                filename.trim_end().to_string(),
            );
        }
        3 => {
            let mut filename = String::new();
            io::stdin()
                .read_line(&mut filename)
                .expect("Failed to read input.");
            info!("{}", filename);
            info!("Enter wallet password: ");
            let mut pass = String::new();
            io::stdin()
                .read_line(&mut pass)
                .expect("Failed to read input.");
            //decryptit
            open_wallet(pass, filename);
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
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    // Convert to an i32.
    let input: i32 = input.trim().parse().unwrap();
    command_control(input);
}
pub fn get_input_wallet() {
    let mut input = String::new();
    // Reads the input from STDIN and places it in the String named input.
    info!("Enter a value:");
    io::stdin()
        .read_line(&mut input)
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

    // get argument from command line
    let args: Vec<String> = env::args().collect();
    // if there is gui arg
    if args.len() > 1 {
        if args[1] == "gui" {
            setup_logging(3).unwrap();

            let app = app::App::default();
            let mut wind = Window::new(100, 100, 400, 300, "Redstone GUI Wallet v0.1");
            let mut frame = Frame::new(0, 0, 400, 300, "");
            let mut pass = SecretInput::new(150, 60, 100, 40, "Password");
            let mut pik = SecretInput::new(150, 10, 100, 40, "Private Key");
            let mut file = Input::new(150, 110, 110, 40, "Filename");
            let mut but = Button::new(30, 210, 100, 40, "Create Wallet");
            let mut but3 = Button::new(150, 210, 100, 40, "Import key");
            let mut but2 = Button::new(270, 210, 100, 40, "Import Wallet");
            wind.end();
            wind.show();
            let mut file1 = file.clone();
            let mut file2 = file.clone();

            let mut pass1 = pass.clone();
            let mut pass2 = pass.clone();

            let mut pik1 = pik.clone();
            but3.set_callback(move |_| {
                let wallet = redstone_rs::keypair::Keypair::from_private_key(pik1.value().parse().unwrap());
                save_wallet(wallet.private_key, pass2.value(), file2.value());
            });

            but.set_callback(move |_| gen_keypair_gui(pass.value().parse().clone().unwrap(),file.value().parse().clone().unwrap()));
            but2.set_callback(move |_| open_wallet_gui(pass1.value().parse().unwrap(),file1.value().parse().unwrap()));


            app.run().unwrap();
        
        }
        if args[1] == "cli" {
            setup_logging(3).unwrap();
            //start logging
            let art = " 
            ██████╗ ███████╗██████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗    ██╗    ██╗ █████╗ ██╗     ██╗     ███████╗████████╗
            ██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝    ██║    ██║██╔══██╗██║     ██║     ██╔════╝╚══██╔══╝
            ██████╔╝█████╗  ██║  ██║███████╗   ██║   ██║   ██║██╔██╗ ██║█████╗      ██║ █╗ ██║███████║██║     ██║     █████╗     ██║   
            ██╔══██╗██╔══╝  ██║  ██║╚════██║   ██║   ██║   ██║██║╚██╗██║██╔══╝      ██║███╗██║██╔══██║██║     ██║     ██╔══╝     ██║   
            ██║  ██║███████╗██████╔╝███████║   ██║   ╚██████╔╝██║ ╚████║███████╗    ╚███╔███╔╝██║  ██║███████╗███████╗███████╗   ██║   
            ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝     ╚══╝╚══╝ ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝   ╚═╝   
            ";
            println!("{}", art);
        
            main_not_logged()
        }
    }

}
