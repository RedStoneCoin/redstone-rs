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

#[get("/test")]
fn hello() -> &'static str {
    //output everyting
    "{ \"success\": true, \"response\": \"Redstone_Node\" }"
}
#[get("/ping")]
fn ping() -> &'static str {
    //output everyting
    "{ \"success\": true, \"response\": \"Pong!\" }"
}
pub fn get_middleware() -> Vec<Route> {
    routes![must_provide_method,
            hello,
            ping
    ]
}
pub fn start_api() {
    let config = Config::build(Environment::Staging)
        .log_level(LoggingLevel::Critical) // disables logging
        .finalize()
        .unwrap();

    rocket::custom(config)
        .mount("/json_api/", get_middleware())
        .launch();

}