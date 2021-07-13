use crate::{crypto::Hashable, executable::Executable};

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
#[derive(Clone, Debug, Default)]
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
    pub fn find_pow(&mut self, difficulty: u64) {
        for nonce_attempt in 0..=u64::MAX {
            self.nonce = nonce_attempt;
            if u64::from_str_radix(self.hash_item().trim_start_matches("0x"), 16).unwrap()
                > difficulty
            {
                self.hash = self.hash_item();
                println!(
                    "Found solution for difficulty {}, nonce {}, hash {}, hash value {}",
                    difficulty,
                    self.nonce,
                    self.hash,
                    u64::from_str_radix(self.hash.trim_start_matches("0x"), 16).unwrap()
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
    fn execute(&self, context: &String) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }

    /// # Evalulate
    /// Checks if a txn is valid
    fn evalute(&self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    /// # Cost
    /// Calculates the txns fee
    fn cost(&self, context: &String) -> u64 {
        todo!()
    }
}
