use crate::blockchain::DATABASE_PATH_PREFIX;
use crate::mempool::get_transaction;
use crate::{
    account::Account, block::Block, blockchain::Blockchain, database::Database, keypair::Keypair,
    mempool::Mempool,
};
use crate::{crypto::Hashable, executable::Executable, state::GlobalState};
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
        

        // finish transaction
        todo!()
    }

    /// # Evalulate 
    /// Checks if a txn is valid
    /// Leo port this to the global state thing
    fn evalute(&self) -> Result<(), Box<dyn std::error::Error>> {
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
        for chn in 0..5 {
            let mut db_handle = Database::new();
            db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, chn))?;
            let block_txn_is_in = db_handle.get(&"transactions".to_owned(), &self.hash);
            if block_txn_is_in.len() == 0 {
            } else {
                return Err("Transaction already in block").unwrap();
            }        
        }
        if pow_txn.starts_with("0000") {
        } else {
            // Proof of work is invalid
            return Err("ErrInvalidPow").unwrap();
        }
        if pow_txn != self.hash_item() {
            // Proof of work is invalid
            return Err("ErrInvalidPow").unwrap();
        }
        todo!()
    }

    /// # Cost
    /// Calculates the txns fee
    fn cost(&self, context: &String) -> u64 {
        todo!()
    }
}
