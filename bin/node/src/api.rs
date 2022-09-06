#![feature(proc_macro_hygiene, decl_macro)]

use crate::File;
use crate::Path;
use lazy_static::*;
use log::*;
use redstone_rs::account::Account;
use redstone_rs::keypair::Keypair;
use redstone_rs::transaction::Transaction;
use redstone_rs::*;
use reqwest::Client;
use rocket::config::{Config, Environment, LoggingLevel};
use rocket::fairing::AdHoc;
use rocket::get;
use rocket::http::hyper::header::AccessControlAllowOrigin;
use rocket::http::hyper::header::Headers;
use rocket::post;
use rocket::{routes, Route};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
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
pub fn submit_txn_v1(txn_data: Json<Transaction>) -> String {
    debug!("Transaction recived by api!");
    let txn = txn_data.0;
    if let Ok(_) =  mempool::add_transaction(txn) {
        return "{ \"result\" : \"sent txn\" }".to_owned();
    } else {
        return "{ \"result\" : \"FAILURE PLEASE TRY LATER\" }".to_owned();
    }
} 


#[get("/get_mem_tx/<hash>")]
fn gettx(hash: String) -> String {
    if let Err(get1) = mempool::get_transaction(hash.clone()) {
        return "{ \"result\" : \"failure\" }".to_owned();
    } else {
        let get = serde_json::to_string(&mempool::get_transaction(hash.clone()).unwrap());
        let return_string = format!("{{ \"result\" : \"{}\" }}", get.unwrap());
        return return_string;
    }
}

#[get("/get_block/<hash>")]
fn get_blk(hash: String) -> String {
    // TODO: Added in blockchain.rs
    let mut result = "{ \"result\" : \"failure\" }".to_owned();
    return result.to_string();
}

#[get("/get_acc/<public_key>")]
fn getacc(public_key: String) -> String {
    if let Err(get1) = Account::get(public_key.clone()) {
        return "{ \"result\" : \"failure\" }".to_owned();
    } else {
        let get = serde_json::to_string(&Account::get(public_key).unwrap());
        let return_string = format!("{{ \"result\" : \"{}\" }}", get.unwrap());
        return return_string;
    }
}
#[get("/sign/<pik>/<hash>")]
fn sign(pik: String,hash: String) -> String {
    let return_string = "Unisecure, delited from api";
    return return_string.to_string();
}


#[get("/create_wallet")]
fn create_wallet() -> String {
    let mut headers = Headers::new();
    headers.set(AccessControlAllowOrigin::Any);
    let wallet = redstone_rs::keypair::Keypair::generate();
    let public_key = wallet.public_key.to_string();
    let private_key = wallet.private_key.to_string();
    let address = wallet.address().to_string();
    return format!(
        "{{ \"public_key\": \"{}\", \"private_key\": \"{}\", \"address\": \"{}\" }}",
        public_key, private_key, address
    );
}
#[get("/from_private_key/<pik>")]
fn from_private_key(pik: String) -> String {
    let keypair = keypair::Keypair::from_private_key(pik);
    let public_key = keypair.public_key.to_string();
    let private_key = keypair.private_key.to_string();
    let address = keypair.address().to_string();
    return format!(
        "{{ \"public_key\": \"{}\", \"private_key\": \"{}\", \"address\": \"{}\" }}",
        public_key, private_key, address
    );
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
    routes![
        must_provide_method,
        ping,
        submit_txn_v1,
        gettx,
        getacc,
        create_wallet,
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
pub fn create_token() {
    info!("Creating api token");
    // token = ./datadir/token.api
    let path = "./datadir/token.api";
    let token: i32 = rand::random();
    let mut file = File::create(path).unwrap();
    let token_write = format!("{}", token);
    // remove -
    let token_write = token_write.replace("-", "");
    file.write_all(&token_write.as_bytes()).unwrap();
    info!("API Token: {}", token_write);
}
pub fn start_api() {
    // Add api token
    // chcek for token
    // if token is not present
    // generate token

    if !Path::new("./datadir/token.api").exists() {
        create_token();
    }
    let token = fs::read_to_string("./datadir/token.api").unwrap();

    let config = Config::build(Environment::Staging)
        .log_level(LoggingLevel::Critical) // disables logging
        .finalize()
        .unwrap();
    // Header apikey
    let url = format!("/json_api/{}/", token.to_string());
    info!("API mounted on {} and key {}", url, token);
    rocket::custom(config)
        .mount(&url, get_middleware())
        .launch();
}
