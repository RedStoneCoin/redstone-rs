use crate::{crypto::Hashable, transaction::Transaction};

pub struct Header {
    height: u64,
    timestamp: u64,
    chain: u64,
    parent_hash: String,
    state_hash: String,
    uncle_root: String,
    proposer: String, // the publickey of the proposer
    transactions_merkle_root: String,
    header_payload: u8,
}
pub struct Block {
    hash: String,
    header: Header,
    transactions: Vec<String>,
    signature: String, // proposers signature
}
impl Hashable for Header {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.height.to_be_bytes().iter());
        bytes
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.header.bytes().iter());
        bytes
    }
}
impl Block {
    pub fn hash(&self) -> String {
        self.hash_item()
    }
    pub fn hash_mut(&mut self) {
        self.hash = self.hash_item()
    }
    pub fn add_txn(&mut self, txn: &Transaction) {
        self.transactions.push(txn.hash.to_owned());
    }
}
