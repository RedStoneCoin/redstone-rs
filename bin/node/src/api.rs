#![feature(proc_macro_hygiene, decl_macro)]

use redstone_rs::*;
use log::*;
use rocket::{routes, Route};
use std::io::prelude::*;
use lazy_static::*;
use serde::Deserialize;
use std::sync::Mutex;
use reqwest::Client;
use std::time::{UNIX_EPOCH, SystemTime};
use std::error::Error;
use rocket::get;
use rocket::post;
use redstone_rs::transaction::Transaction;
use redstone_rs::account::Account;
use redstone_rs::keypair::Keypair;
use rocket::http::hyper::header::Headers;
use rocket::http::hyper::header::AccessControlAllowOrigin;
use rocket::fairing::AdHoc;
use std::fs;
use rocket::config::{Config, Environment, LoggingLevel};
lazy_static! {
    static ref WALLET_DETAILS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}
#[derive(Default, Clone, Deserialize, Debug)]
struct TxnDetails {
    pub amount: u64,
    pub reciever: String,
    pub sender: String,
    pub txn_type: String,
    pub extra: String,
}

#[derive(Clone, Deserialize, Debug)]
struct Blockcount {
    success: bool,
    blockcount: u64,
}

#[derive(Clone, Deserialize, Debug)]
struct HashAtHeight {
    success: bool,
    hash: String,
}

#[derive(Clone, Deserialize, Debug)]
struct Transactioncount {
    success: bool,
    transaction_count: u64,
}
#[get("/")]
fn must_provide_method() -> &'static str {
    "{ \"success\": false, \"error\": \"METHOD_MISSING\" }"
}

#[get("/ping")]
fn ping() -> &'static str {
    //output everyting
    "{ \"success\": true, \"response\": \"Pong!\" }"
}
pub fn to_atomc(amount: f64) -> u64 {
    (amount * (10_i64.pow(6 as u32) as f64)) as u64 // (amount * 10000 for 4 dec places)
}

pub fn to_dec(amount: u64) -> u64 {
    if amount == 0 {
        return 0 as u64;
    } else {
        return amount as u64 / (10_i64.pow(6 as u32)) as u64;
        // (amount / 10000 for 4 dec places)
    }
}

#[post("/submit_txn", format = "application/json", data = "<txn_data>")]
pub fn submit_txn_v1(txn_data: rocket::Data) -> String {
    debug!("Transaction recived by api!");
    let mut holder_vec: Vec<u8> = vec![];
    let mut txn_data1 = txn_data.open();

    loop {
        let mut buffer = [0u8; 512];
        let try_read_from_stream = txn_data1.read(&mut buffer);
        if let Ok(size) = try_read_from_stream {
            trace!("Read {} bytes into buffer", size);
            if size == 0 {
                break;
            } else {
                holder_vec.append(&mut buffer.to_vec());
            }
        } else {
            debug!(
                "Failed to read into buf, error={}",
                try_read_from_stream.unwrap_err()
            );
            return format!(" {{ \"error\" : \" failed to read from datastream \" }}");
        }
    }
    let try_utf8_to_json = String::from_utf8(holder_vec);
    if let Ok(txn_pretrim) = try_utf8_to_json {
        if txn_pretrim != "" {
            let mut txn = txn_pretrim[1..].replace("\\", "").to_string(); // this very verbose bit of code removes everything outside the { } and removes the \
            loop {
                if &txn[txn.len() - 1..] != "}" {
                    txn = txn[0..txn.len() - 1].to_string();
                } else {
                    break;
                }
            }
            trace!("Txn submited by API json={}", txn);
            let try_string_to_txn = serde_json::from_str::<Transaction>(&txn);
            if let Ok(txn1) = try_string_to_txn {
                mempool::add_transaction(txn1);
                return "{ \"result\" : \"sent txn\" }".to_owned();
            } else {
                return "{ \"result\" : \"FAILURE PLEASE TRY LATER\" }".to_owned();
            }
        }
        else {
            debug!(
              "Failed to turn utf8 bytes to txn (submit txn api, error={})",
              "no"
            );
        return format!(" {{ \"error\" : \" utf8 to json failed \" }}");
        }
  }else {
        return "{ \"result\" : \"failure\" }".to_owned();
  }
}



#[post("/submit_txn_np", format = "application/json", data = "<txn_data>")]
pub fn submit_txn_v1_np(txn_data: rocket::Data) -> String {
    debug!("Transaction recived by api!");
    let mut holder_vec: Vec<u8> = vec![];
    let mut txn_data1 = txn_data.open();

    loop {
        let mut buffer = [0u8; 512];
        let try_read_from_stream = txn_data1.read(&mut buffer);
        if let Ok(size) = try_read_from_stream {
            trace!("Read {} bytes into buffer", size);
            if size == 0 {
                break;
            } else {
                holder_vec.append(&mut buffer.to_vec());
            }
        } else {
            debug!(
                "Failed to read into buf, error={}",
                try_read_from_stream.unwrap_err()
            );
            return format!(" {{ \"error\" : \" failed to read from datastream \" }}");
        }
    }
    let try_utf8_to_json = String::from_utf8(holder_vec);
    if let Ok(txn_pretrim) = try_utf8_to_json {
        if txn_pretrim != "" {
            let mut txn = txn_pretrim[1..].replace("\\", "").to_string(); // this very verbose bit of code removes everything outside the { } and removes the \
            loop {
                if &txn[txn.len() - 1..] != "}" {
                    txn = txn[0..txn.len() - 1].to_string();
                } else {
                    break;
                }
            }
            trace!("Txn submited by API json={}", txn);
            let try_string_to_txn = serde_json::from_str::<Transaction>(&txn);
            if let Ok(mut txn1) = try_string_to_txn {
                let pow = txn1.find_pow();
                mempool::add_transaction(txn1);
                return "{ \"result\" : \"sent txn\" }".to_owned();
            } else {
                return "{ \"result\" : \"FAILURE PLEASE TRY LATER\" }".to_owned();
            }
        }
        else {
            debug!(
              "Failed to turn utf8 bytes to txn (submit txn api, error={})",
              "no"
            );
        return format!(" {{ \"error\" : \" utf8 to json failed \" }}");
        }
  }else {
        return "{ \"result\" : \"failure\" }".to_owned();
  }
}
#[get("/get_mem_tx/<hash>")]
fn gettx(hash: String) -> String {
    if let Err(get1) = mempool::get_transaction(hash.clone())  {
        return "{ \"result\" : \"failure\" }".to_owned();    } 
    else {
        let get = serde_json::to_string(&mempool::get_transaction(hash.clone()).unwrap());
        return "{ \"success\": true, \"Result\":".to_string() + &get.unwrap() + "}";

    }
}

#[get("/get_block/<hash>")]
fn get_blk(hash: String) -> String {
    
    // TODO
    let mut result = "{ \"result\" : \"failure\" }".to_owned();
    return result.to_string();
}


#[get("/get_acc/<public_key>")]
fn getacc(public_key: String) -> String {
    if let Err(get1) = Account::get(public_key.clone())  {
        return "{ \"result\" : \"failure\" }".to_owned();    } 
    else {
        let get = serde_json::to_string(&Account::get(public_key).unwrap());
        return "{ \"success\": true, \"Result\":".to_string() + &get.unwrap() + "}";
    }
}
#[get("/sign/<pik>/<hash>")]
fn sign(pik: String,hash: String) -> String {
    let keypair = Keypair {
        private_key: pik.clone(),
        public_key: hash.clone(),
    };
    let sig = keypair.sign(hash.clone());
    return "{ \"success\": true, \"Result\":".to_string() + &sig.unwrap() + "}";

}

#[get("/send_easy_transaction/<pik>/<from>/<amount>/<to>")]
fn es_tx(pik: String,from: String,amount: f64,to: String) -> String {
    let keypair = Keypair {
        private_key: pik.clone(),
        public_key: from.clone(),
    };
    let mut txn1 = Transaction {
        hash: "".to_owned(),
        sender: from.clone().to_owned(),
        reciver: to.clone().to_owned(),
        amount: to_atomc(amount).to_owned(),
        nonce: 0,
        type_flag: 0,
        payload: "".to_owned(), // Hex encoded payload
        pow: "".to_owned(), // Spam protection PoW
        signature: "".to_owned(),
    };                    //99999999999999999999
    let pow = txn1.find_pow();
    let sig = keypair.sign(txn1.hash.clone());
    txn1.signature = sig.unwrap();
    mempool::add_transaction(txn1);
    return "{ \"success\": true, \"Result\":".to_string() + "Not failure" + "}";
}

#[get("/create_wallet")]
fn create_wallet() -> String {
    let mut headers = Headers::new();
    headers.set(
        AccessControlAllowOrigin::Any
    );
    let wallet = redstone_rs::keypair::Keypair::generate();
    let public_key = wallet.public_key.to_string();
    let private_key = wallet.private_key.to_string();
    let address = wallet.address().to_string();
    return format!("{{ \"public_key\": \"{}\", \"private_key\": \"{}\", \"address\": \"{}\" }}", public_key, private_key, address);
}
#[get("/from_private_key/<pik>")]
fn from_private_key(pik: String) -> String {
    let keypair = keypair::Keypair::from_private_key(pik);
    let public_key = keypair.public_key.to_string();
    let private_key = keypair.private_key.to_string();
    let address = keypair.address().to_string();
    return format!("{{ \"public_key\": \"{}\", \"private_key\": \"{}\", \"address\": \"{}\" }}", public_key, private_key, address);
}

#[get("/pk_to_acc/<pik>")]
fn pkacc(pik: String) -> String {
    let keypair = Keypair {
        private_key: pik.clone(),
        public_key: pik.clone(),
    };
    let addr = keypair.address().to_string();
    // check for errors
    return format!("{{ \"address\": \"{}\" }}", addr);
}
pub fn get_middleware() -> Vec<Route> {
    routes![must_provide_method,
            ping,
            submit_txn_v1,
            gettx,
            getacc,
            create_wallet,
            submit_txn_v1_np,
            es_tx,
            from_private_key,
            pkacc
    ]
}


// header token must be provided
// token = ./datadir/token.api

pub fn get_token() {
    let token = fs::read_to_string("./datadir/token.api").unwrap();
    token.to_string();
}

pub fn start_api() {
    // Add api token
    let token = fs::read_to_string("./datadir/token.api").unwrap();
    
    let config = Config::build(Environment::Staging)
        .log_level(LoggingLevel::Critical) // disables logging
        .finalize()
        .unwrap();
    // Header apikey
    let url = format!("/json_api/{}/", token.to_string());
    info!("API mounted on {}", url);
    rocket::custom(config)
        .mount(&url, get_middleware())
        .launch();

}