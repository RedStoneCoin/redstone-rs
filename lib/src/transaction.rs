use crate::{crypto::Hashable, executable::Executable};
use serde::{Deserialize, Serialize};
use crate::state::GlobalState;
use crate::{keypair::Keypair,account::Account};
pub enum TxType{
    Send = 0, // used to send funds
    Burn =  1, // used to destroy funds
    ToggleOnline = 2, // used for a validator to go offline
    EvidenceTxn = 3, // Used to report voting on multiple chains
    DelegateTx = 4, // Used to delegate staking power to another node (or undelegate)
    CallContract = 5, // Used to call a contract's assosiated functions
    CreateChain = 6, // Used to create a new chain (when authorised)
    Coinbase = 7, // The first txn in any block, creates coins to send to the validators and proposer

}
#[derive(Deserialize,Serialize,Clone, Debug, Default)]
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
    pub fn new(sender: String,reciver: String,amount: u64,type_flag: u8,payload: String,) -> Self {
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
        for nonce_attempt in 0..=u64::MAX {
            self.nonce = nonce_attempt;
            let pow = self.hash_item();
            //println!("pow test: {}",pow);
            if pow.starts_with("0000")
            {
                self.pow = self.hash_item();
                self.hash = self.hash_item();
                self.nonce = nonce_attempt;
                println!("Found solution for , nonce {}, hash {}, hash value {}",self.nonce,self.hash,pow);
                break;
            }
            
        }
    }
}


impl Executable for Transaction {
    /// # Execute
    /// Executes this transaction, updating the account balances and executing all smart contracts touched
    /// Returns the error code encountered OR the new account state hash
    fn execute(&self, context: &String,state: &mut GlobalState) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }

    /// # Evalulate
    /// Checks if a txn is valid
    fn evalute(&self) -> Result<(), Box<dyn std::error::Error>> {
        let keypairs = Keypair {
            public_key: self.sender.clone(),
            private_key: "".to_string(),
        };
        let check = keypairs.verify(
            &self.hash, 
            &self.signature
        );
        let pow_txn = self.hash_item();
        let db_txn = "";
        
        if check.is_ok() {
            // Signature is valid
            if pow_txn == self.hash {
            // Proof of work is valid
                if self.hash != db_txn {
                    // Transaction is original
                    let acc_sender = Account::get(self.sender.clone());
                    let acc_reciver = Account::get(self.reciver.clone());

                    if self.amount < Account::get(self.sender.clone()).unwrap().balance {
                        // Transaction is valid
                        acc_sender.unwrap().balance -= self.amount;
                        acc_reciver.unwrap().balance += self.amount;
                        return Ok(());
                    } else {
                        // Transaction is invalid
                        return Err("Transaction amount is greater than sender's balance").unwrap();
                    }
                } 
                else {
                    // Transaction is not original
                    return Err("Transaction is not original").unwrap();
                }
                } 
            } 
            else {
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
