#![feature(proc_macro_hygiene, decl_macro)]


use fern::colors::{Color, ColoredLevelConfig};
use log::*;
use redstone_rs::*;
mod api;
use std::collections::HashMap;
use redstone_rs::block::{Header,Block};
use redstone_rs::transaction::Transaction;
use redstone_rs::account::Account;
use redstone_rs::crypto::hash;
use redstone_rs::rs_p2p::p2p;
use std::path::PathBuf;
extern crate clap;
use clap::{Arg, App, SubCommand};
use redstone_rs::rpc::{block_announce, Announcement, Caller};
use std::{thread, time};
use std::path::Path;
use std::fs;
use redstone_rs::blockchain::Blockchain;
use fs::File;
use std::io::Write;
use redstone_rs::config::Config;
extern crate tokio;
extern crate rand;

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

fn test_all() {
        // crete test chain
        if true {
            // loop so program does not end
            /*
            let _ = std::thread::spawn(move || {
                let mut txn = Transaction {
                    hash: "321".to_owned(),
                    sender: "0302db1c230c9e215a2cb251a2b08af301c8046661298de92b3a250fcf682d36".to_owned(),
                    reciver: "0x1530fc2f2364e35f1408087119b497e3ea324d5c".to_owned(),
                    amount: 69,
                    nonce: 1,
                    type_flag: 0,
                    payload: "".to_owned(), // Hex encoded payload
                    pow: "".to_owned(),     // Spam protection PoW
                    signature: "".to_owned(),
                };
                let mut txn1 = Transaction {
                    hash: "123".to_owned(),
                    sender: "02638a3e97620e1e9fc7127e2644815bc33ab03ad7e47c525f86a92ef7eac3b09f".to_owned(),
                    reciver: "0x1530fc2f2364e35f1408087119b497e3ea324d5c".to_owned(),
                    amount: 29,
                    nonce: 1,
                    type_flag: 0,
                    payload: "".to_owned(), // Hex encoded payload
                    pow: "".to_owned(),     // Spam protection PoW
                    signature: "".to_owned(),
                };
                let acc = Account {
                    address: "0x1530fc2f2364e35f1408087119b497e3ea324d5c".to_owned(),
                    balance: 0,
                    smart_contract: false,
                    nonce: 0,
    
                };
                account::Account::save(&acc);
                let mut blk = Block {
                    hash: "02638a3e97620e1e9fc7127e2644815bc33ab03ad7e47c525f86a92ef7eac3b09f".to_owned(),
                    header: Header {
                        height: 66,
                        timestamp: 1,
                        chain: 2,
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
                    transactions: vec![txn.clone(), txn1.clone()],
                };
                
                // get blocks form db and send them to the wallet to sync it
                //  let block = vec![blk,blk1,blk2];
                info!("wait 5 sec");
                thread::sleep(time::Duration::from_secs(5));
                info!("announe block test1 ");
                block_announce(blk).unwrap();
                thread::sleep(time::Duration::from_secs(1));
                
            });
            */
        }
    
}
fn start_node(rpc_port: u64,test: bool,api: bool,private_key: String,validator: String,config: Config) {
    // TODO move to config file
    let ver = "0.0.1";
    setup_logging(3).unwrap();

    let assci_art = "
    ██████╗ ███████╗██████╗ ███████╗████████╗ ██████╗ ███╗   ██╗███████╗    ███╗   ██╗ ██████╗ ██████╗ ███████╗
    ██╔══██╗██╔════╝██╔══██╗██╔════╝╚══██╔══╝██╔═══██╗████╗  ██║██╔════╝    ████╗  ██║██╔═══██╗██╔══██╗██╔════╝
    ██████╔╝█████╗  ██║  ██║███████╗   ██║   ██║   ██║██╔██╗ ██║█████╗      ██╔██╗ ██║██║   ██║██║  ██║█████╗  
    ██╔══██╗██╔══╝  ██║  ██║╚════██║   ██║   ██║   ██║██║╚██╗██║██╔══╝      ██║╚██╗██║██║   ██║██║  ██║██╔══╝  
    ██║  ██║███████╗██████╔╝███████║   ██║   ╚██████╔╝██║ ╚████║███████╗    ██║ ╚████║╚██████╔╝██████╔╝███████╗
    ╚═╝  ╚═╝╚══════╝╚═════╝ ╚══════╝   ╚═╝    ╚═════╝ ╚═╝  ╚═══╝╚══════╝    ╚═╝  ╚═══╝ ╚═════╝ ╚═════╝ ╚══════╝                                                                                                       
";
    info!("{}",assci_art);
    warn!("Please enter 'exit' to shut down");
    info!("Starting redstone node {}" , ver);
    warn!("Warning, this software is not stable");
    warn!("Run at your own risk!");
    // sleep for a while to let the user read the warning
    if validator != "" {
        let wallet = redstone_rs::keypair::Keypair::from_private_key(private_key);
        info!("Starting VALIDATOR NODE");
        info!("Validator: {}",wallet.address());
    }
    mempool::Mempool::init(HashMap::new()).unwrap();
    // genesis chains blockchain.rs
    // create if blockchain_db_0-4 does not exist
    let mut exist_chains = 0;
    for i in 0..5 {
        let mut path = PathBuf::from("./datadir");
        path.push(format!("blockchain_db_{}", i));
        // if any of 5 chains does not exist set exist_chains to true or false
        if path.exists() {
            exist_chains = 1;
        }
    }

    if exist_chains == 0 {
        info!("CREATING GENESIS CHAINS");
        blockchain::Blockchain::create_genesis_blockchains().unwrap();
        info!("CREATED GENESIS CHAINS");
        for i in 0..5 {
            let bc = blockchain::Blockchain::new(i);
            let test_block = bc.get_block_by_height(&0);
            info!("GENESSIS BLOCK: {:?} CHAIN: {}",test_block,i);
        }
    } else {
        info!("GENESIS CHAINS ALREADY EXIST");
    }

    if api {
    info!("Launching API server at 0.0.0.0:8000");
    let _ = std::thread::spawn(move || {
        api::start_api();
    });
    } else {
        info!("API DISABLED!!!");
    }
    let _ = std::thread::spawn(move || {
        redstone_rs::rpc::launch(rpc_port);
    });
    // check if datadir exists
    let datadir = "./datadir";
    if !Path::new(datadir).exists() {
        info!("Creating datadir");
        fs::create_dir(datadir).unwrap();
    }
    info!("Launching P2P server");
    //launch async redstone_rs::rs_p2p::server::start_server()
    let config1 = config.clone();
    let config2 = config.clone();
    let _ = std::thread::spawn(move || {
        let mut port = config1.p2p_port().clone();
        let mut bootnode = config1.bootnode().clone();
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            redstone_rs::rs_p2p::p2p::start(port.into(),bootnode.clone()).await;
        });
    });
    let _ = std::thread::spawn(move || {
        let mut port = config2.p2p_port().clone();
        let mut bootnode = config2.bootnode().clone();
        let mut rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            redstone_rs::rs_p2p::p2p::start_other(port.into(),bootnode.clone()).await;
        });
    });
    loop {         
         // dont exit loop, if removed node wont work
         // -- Founder - Nov 26 '21 at 10:37
        // new loop, if ctrl+c is pressed, it will exit the loop
        // -- Founder - Nov 26 '21 at 10:37
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit" {
            // shut down all threads
            info!("Shutting down");
            std::process::exit(0);
        } else {
            warn!("Please enter 'exit' to shut down");
        }

    }
}
fn main() {
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
                          .arg(Arg::with_name("api_port")
                            .long("api_port") // allow --name
                            .takes_value(true)
                            .help("api port")
                            .required(false))
                        .arg(Arg::with_name("rpc_port")
                            .long("rpc") // allow --name
                            .takes_value(true)
                            .help("rpc port")
                            .required(false))
                        // p2p port
                        .arg(Arg::with_name("p2p_port")
                            .long("p2p") // allow --name
                            .takes_value(true)
                            .help("p2p port")
                            .required(false))
                        // testnet
                        .arg(Arg::with_name("testnet")
                            .long("testnet") // allow --name
                            .takes_value(false)
                            .help("testnet")
                            .required(false))
                        .arg(Arg::with_name("private_key")
                            .long("private_key") // allow --name
                            .takes_value(true)
                            .help("private_key")
                            .required(false))
                
                          .get_matches();
    
    // return vec of args

    let mut rpc_port = matches.value_of("rpc_port").unwrap_or("").to_string();
    let api_port = matches.value_of("api_port").unwrap_or("").to_string();
    let mut validator = matches.value_of("validator").unwrap_or("").to_string();
    let mode = matches.value_of("mode").unwrap_or("").to_string();
    let logging = matches.value_of("logging").unwrap_or("").to_string();
    let mut testnet = matches.is_present("testnet");
    let mut api = matches.is_present("no_api");
    let private_key = matches.value_of("private_key").unwrap_or("").to_string();
    let mut p2p_port = matches.value_of("p2p_port").unwrap_or("").to_string();
    // if rpc_port is empty set it to 44405

    
    if rpc_port.is_empty() {
        rpc_port = "44405".to_string();
    }
    if p2p_port.is_empty() {
        p2p_port = "44404".to_string();
    }

    if testnet == true {
        testnet = true;
    }
    // pub struct Config {
    //     p2p_port: u16,
    //     rpc_port: u16,
    //     bootnode: String,
    // }
    let bootnode = format!("{}:{}", "127.0.0.1", "44404").to_string();
    let config = Config::new(p2p_port.parse::<u16>().unwrap(),rpc_port.parse::<u16>().unwrap(),bootnode);
    if !validator.is_empty() && private_key.is_empty() {
        println!("Private key is required for validator");
        return;
    }
    start_node(rpc_port.parse::<u16>().unwrap().into(),testnet,true,private_key,validator,config)
}
