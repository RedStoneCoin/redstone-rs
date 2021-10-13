use std::fmt::format;
use serde::Serialize;

use crate::database::Database;
#[derive(Clone, Default, Debug, PartialEq,Serialize)]
pub struct Account {
    pub address: String,
    pub balance: u64,
    pub smart_contract: bool,
}
impl Account {
    pub fn get(address: String) -> Result<Account, Box<dyn std::error::Error>> {
        let mut database_handle = Database::new();
        database_handle.open(&"accounts".into())?;
        let encoded = database_handle.get(&"accounts".into(), &address);
        if encoded.len() == 0 {
            println!("{:?}",encoded);
            return Err("Poor formating or address not found".into());
        } else {
            return Account::from_string(String::from_utf8(hex::decode(encoded)?)?);
        }
    }
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let encoded = self.acc_string();
        let mut database_handle = Database::new();
        database_handle.open(&"accounts".into())?;
        database_handle.set(&"accounts".into(), &self.address, &encoded)

    }
    pub fn from_string(encoded: String) -> Result<Account, Box<dyn std::error::Error>> {
        let split_string = encoded.split('.').collect::<Vec<&str>>();
        if split_string.len() != 3 {
            return Err("Poor formating. 2".into());
        } else {
            Ok(Account {
                address: split_string[0].to_string(),
                balance: split_string[1].parse()?,
                smart_contract: split_string[2].parse()?,
            })
        }
    }
    pub fn acc_string(&self) -> String {
        let raw = format!("{}.{}.{}", self.address, self.balance, self.smart_contract);
        hex::encode(raw)
    }
}
