use crate::{blockchain::Blockchain, crypto::Hashable, executable::Executable, state::GlobalState, transaction::Transaction};

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
    proof: String, // The vrf proof of the proposer as hex
    proposer_signature: String, // proposers signature
    validator_signatures: Vec<String>
}
pub struct Block {
    hash: String,
    header: Header,
    transactions: Vec<String>,
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

impl Executable for Block {
    fn execute(&self, context: &String) -> Result<String, Box<dyn std::error::Error>> {
        todo!()
    }

    fn evalute(&self) -> Result<(), Box<dyn std::error::Error>> {
        // check the hash of this block
        if self.hash != self.hash_item() {
            return Err("Hash invalid".into());
        }
        // check the chain exists
        if let Ok(bc) = Blockchain::load(self.header.chain) {
            if self.header.height == 0 {
                // if this block is the genesis block of this chain, the parent block should contain a create chain TXN
                // TODO: Get parent block and look for this txn
            } else {
                // check if the height and parent hash is correct on the specified chain
                let chain_tip = bc.tip();
                if self.header.parent_hash != chain_tip {
                    return Err("Invalid parent hash".into());
                } else {
                    // get the parent block and check its height is self.header.height - 1
                    if let Ok(block) = Block::get(chain_tip) {
                        if block.header.height != self.header.height - 1 {
                            return Err("Height missmatch".into());
                        } else {
                            // check the state hash
                            if self.header.state_hash != GlobalState::current() {
                                return Err("Invalid state hash".into())
                            }
                            // TODO: Check uncle root (the merkle root of all the tips of the other chains)
                            // TODO: Check the proposer
                        }
                    }
                }
            }
        } else {
            return Err("Chain does not exist".into());
        }
        Ok(())
    }

    /// # Cost
    /// Not used, will panic if called
    fn cost(&self, context: &String) -> u64 {
        unimplemented!()
    }
}
