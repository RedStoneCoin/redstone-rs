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
use redstone_rs::config;

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

#[post("/test", format = "application/json", data = "<test>")]
fn test(test: String) { /* ... */


}
fn main() {
    rocket::ignite().mount("/", routes![test]).launch();
}