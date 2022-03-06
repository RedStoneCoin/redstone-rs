use crate::blockchain::DATABASE_PATH_PREFIX;
use crate::mempool::get_transaction;
use crate::{
    account::Account, block::Block, blockchain::Blockchain, database::Database, keypair::Keypair,
    mempool::Mempool,
};
use crate::{crypto::Hashable, executable::Executable, state::GlobalState};
use log::warn;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub enum TxType {
    Send = 0,         // used to send funds
    Burn = 1,         // used to destroy funds
    ToggleOnline = 2, // used for a validator to go offline
    EvidenceTxn = 3,  // Used to report voting on multiple chains
    DelegateTx = 4,   // Used to delegate staking power to another node (or undelegate)
    CallContract = 5, // Used to call a contract's assosiated functions
    CreateChain = 6,  // Used to create a new chain (when authorised)
    Coinbase = 7, // The first txn in any block, creates coins to send to the validators and proposer
}
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Transaction {
    pub hash: String,
    pub sender: String,
    pub reciver: String,
    pub amount: u64,
    pub nonce: u64,
    pub type_flag: u8,
    pub payload: String, // Hex encoded payload
    pub pow: String,     // Spam protection PoW
    pub signature: String,
}
pub struct pow {
    pub hash: String,
    pub nonce: u64,
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut out = vec![];
        out.extend(self.sender.bytes());
        out.extend(self.reciver.bytes());
        out.extend(self.amount.to_string().bytes());
        out.extend(self.nonce.to_string().bytes());
        out.push(self.type_flag);
        out.extend(self.payload.bytes());

        out
    }
}

impl Transaction {
    pub fn new(
        sender: String,
        reciver: String,
        amount: u64,
        type_flag: u8,
        payload: String,
    ) -> Self {
        let mut txn = Transaction {
            sender,
            reciver,
            amount,
            type_flag,
            payload,
            nonce: 0,
            pow: String::default(),
            signature: String::default(),
            hash: String::default(),
        };
        txn.hash = txn.hash_item();
        txn
    }
    pub fn find_pow(&mut self) {
        let mut rng = rand::thread_rng();
        for nonce_attempt in 0..=u64::MAX {
            let nonce_attempt = rng.gen::<u64>();
            self.nonce = nonce_attempt;
            let pow = self.hash_item();
            //println!("pow test: {}",pow);
            if pow.starts_with("0000") {
                self.pow = self.hash_item();
                self.hash = self.hash_item();
                self.nonce = nonce_attempt;
                println!(
                    "Found solution for , nonce {}, hash {}, hash value {}",
                    self.nonce, self.hash, pow
                );
                break;
            }
        }
    }
}

impl Executable for Transaction {
    /// # Execute
    /// Executes this transaction, updating the account balances and executing all smart contracts touched
    /// Returns the error code encountered OR the new account state hash
    /// The changes are not applied on disc, but rather we mutate the state object passed to us
    /// This allows rollbacks and in memory applications

    fn execute(
        &self,
        context: &String,
        state: &mut GlobalState,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let keypairs = Keypair {
            public_key: self.sender.clone(),
            private_key: "".to_string(),
        };
        let sender = keypairs.address();
        let reciver = self.reciver.clone();
        let sender_account = Account::get(self.sender.clone())?;

        match self.type_flag {
            0 => {
                // burn
                let mut sender_account = sender_account.clone();
                sender_account.balance -= self.amount;
                sender_account.save()?;
                let mut db_handle = Database::new();

                db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, "txs"))?;
                if let Some(mut tx_count_burn) = db_handle.get(
                    &"transactions".to_owned(),
                    &"transactions_count_burn".to_owned(),
                )? {
                    let mut tx_count_numerical = tx_count_burn.parse::<u64>().unwrap();
                    tx_count_numerical += 1;
                    db_handle.set(
                        &"transactions".to_owned(),
                        &"transactions_count".to_owned(),
                        &tx_count_numerical.to_string(),
                    );
                } else {
                    db_handle.set(
                        &"transactions".to_owned(),
                        &"transactions_count".to_owned(),
                        &"1".to_string(),
                    );
                }
                drop(db_handle);
            }
            1 => {
                // send
                let mut sender_account = sender_account.clone();
                sender_account.balance -= self.amount;
                // check for reciver if exist
                let check = match Account::get(self.reciver.clone()) {
                    Ok(account) => {
                        let mut account = account.clone();
                        account.balance += self.amount;
                        account.save()?;
                    }
                    Err(_) => {
                        // create new account
                        let mut account = Account {
                            address: self.reciver.clone(),
                            balance: self.amount,
                            smart_contract: false,
                        };
                        account.save()?;
                    }
                };
            }
            _ => {
                // return error
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Invalid transaction type",
                )));
            }
        }

        let mut db_handle = Database::new();
        // END OF TRANSACTION execution
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, "txs"))?;
        if let Some(tx_count) =
            db_handle.get(&"transactions".to_owned(), &"transactions_count".to_owned())?
        {
            let mut tx_count_numerical = tx_count.parse::<u64>().unwrap();
            tx_count_numerical += 1;
            db_handle.set(
                &"transactions".to_owned(),
                &"transactions_count".to_owned(),
                &tx_count_numerical.to_string(),
            );
        } else {
            db_handle.set(
                &"transactions".to_owned(),
                &"transactions_count".to_owned(),
                &"1".to_string(),
            );
        }
        // TODO SAVE TX
        db_handle.set(
            &"transactions".to_owned(),
            &self.hash.clone(),
            &self.hash.clone(),
        );
        drop(db_handle);

        todo!()
    }

    /// # Evalulate
    /// Checks if a txn is valid
    fn evalute(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Evaluating transaction hash: {}", self.hash.clone());
        let mpool = get_transaction(self.hash.clone());
        if let Err(ref mpool) = mpool {
        } else {
            println!("{:?}", mpool);
            return Err("Transaction is in mempool").unwrap();
        }
        let keypairs = Keypair {
            public_key: self.sender.clone(),
            private_key: "".to_string(),
        };
        let check = keypairs.verify(&self.hash, &self.signature);
        let pow_txn = self.hash_item();
        if let Err(ref check1) = check {
            println!("{:?}", check);
            return Err("Signature is not valid").unwrap();
        }
        // add check in db txs if there was transaction executed wit hsame hash
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, "txs"))?;
        if let Some(block_txn_is_in) = db_handle.get(&"transactions".to_owned(), &self.hash)? {
            if block_txn_is_in.len() == 0 {
                return Err("Transaction already in block").unwrap();
            }
        } else {
              warn!("Failed to read traansactions contained in block {} from db (key not found)", self.hash);
              return Err("Failed to read any transactions for block from DB (key not found)".into());
        }
        if !pow_txn.starts_with("0000") { // TODO: Use difficulty factor and then remove PoW
            // Proof of work is invalid
            return Err("ErrInvalidPow").unwrap();
        }
        if pow_txn != self.hash_item() {
            // Proof of work is invalid
            return Err("ErrInvalidPow").unwrap();
        }
        if self.sender.len() != 64 {
            return Err("ErrInvalidSender").unwrap();
        }
        if self.nonce > u64::MAX {
            return Err("ErrInvalidNonce").unwrap();
        }
        if self.nonce < 0 {
            return Err("ErrInvalidNonce").unwrap();
        }
        if ![1, 2, 3, 4, 5, 6, 7].contains(&self.type_flag) { // TODO: use a constant rather than this array for supported txn flags
            println!(
                "Transaction {} has unsupported type={}",
                self.hash, self.type_flag,
            );
            return Err("ErrInvalidType").unwrap();
        }
        match self.type_flag {
            0 => {
                // burn
                if self.amount > u64::MAX {
                    return Err("ErrInvalidAmount").unwrap();
                }
                if self.reciver.len() != 64 {
                    return Err("ErrInvalidReciver").unwrap();
                }
                let sender = Account::get(self.sender.clone())?;
                let reciver = Account::get(self.reciver.clone())?;
                match Account::get(self.sender.clone()) {
                    Ok(_) => {
                        if sender.balance < self.amount {
                            return Err("ErrInsufficientFunds").unwrap();
                        }
                    }
                    Err(e) => {
                        return Err("ErrAccountNotFound").unwrap();
                    }
                }
                // finish THIS but for now return ok(())
                return Ok(());
            }
            1 => {
                // send
                if self.amount > u64::MAX {
                    return Err("ErrInvalidAmount").unwrap();
                }
                if self.reciver.len() != 64 {
                    return Err("ErrInvalidReciver").unwrap();
                }
                let sender = Account::get(self.sender.clone())?;
                let reciver = Account::get(self.reciver.clone())?;
                match Account::get(self.sender.clone()) {
                    Ok(_) => {
                        if sender.balance < self.amount {
                            return Err("ErrInsufficientFunds").unwrap();
                        }
                    }
                    Err(e) => {
                        return Err("ErrAccountNotFound").unwrap();
                    }
                }
                // finish THIS but for now return ok(())
                return Ok(());
            }
            2 => {
                // ToggleOnline
                if self.amount > u64::MAX {
                    return Err("ErrInvalidAmount").unwrap();
                }
                if self.reciver.len() != 64 {
                    return Err("ErrInvalidReciver").unwrap();
                }
                let sender = Account::get(self.sender.clone())?;
                let reciver = Account::get(self.reciver.clone())?;
                match Account::get(self.sender.clone()) {
                    Ok(_) => {
                        if sender.balance < self.amount {
                            return Err("ErrInsufficientFunds").unwrap();
                        }
                    }
                    Err(e) => {
                        return Err("ErrAccountNotFound").unwrap();
                    }
                }
                // TODO
            }
            _ => {
                // TODO add other types
                return Err("ErrInvalidType").unwrap();
            }
        }
        todo!()
    }

    /// # Cost
    /// Calculates the txns fee
    fn cost(&self, context: &String) -> u64 {
        todo!()
    }
}
