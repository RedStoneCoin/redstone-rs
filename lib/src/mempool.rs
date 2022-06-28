use crate::executable::Executable;
use crate::transaction::Transaction;
use lazy_static::lazy_static;
use std::{collections::HashMap, sync::Mutex};
lazy_static! {
    static ref MEMPOOL: Mutex<Mempool> = Mutex::new(Mempool::default());
}
use log::*;
#[derive(Clone, Debug, Default)]
pub struct Mempool {
    transactions: HashMap<String, Transaction>,
}

impl Mempool {
    pub fn init(
        transactions: HashMap<String, Transaction>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!(
            "Initialzing mempool with {} transactions",
            transactions.keys().len()
        );
        let mut lock = MEMPOOL.lock()?;
        *lock = Mempool { transactions };
        return Ok(());
    }
}

pub fn add_transaction(tx: Transaction) -> Result<(), Box<dyn std::error::Error>> {
    let org_tx = tx.clone();
    let validate_tx = tx.evalute();
    if let Err(e) = validate_tx {
        return Err(e);
    } else {
        let mut lock = MEMPOOL.lock()?;
        lock.transactions.insert(tx.hash.clone(), tx);
        info!("Transaction added to the mempool: {:#?}", org_tx);
        Ok(())
    }
}

pub fn get_transaction(hash: String) -> Result<Transaction, Box<dyn std::error::Error>> {
    let lock = MEMPOOL.lock()?;
    if let Some(tx) = lock.transactions.get(&hash) {
        return Ok(tx.clone());
    } else {
        return Err("tx not in mempool".into());
    }
}

pub fn remove_transaction(hash: String) -> Result<Transaction, Box<dyn std::error::Error>> {
    let mut lock = MEMPOOL.lock()?;
    if let Some(tx) = lock.transactions.remove(&hash) {
        return Ok(tx);
    } else {
        return Err("tx not in mempool".into());
    }
}
