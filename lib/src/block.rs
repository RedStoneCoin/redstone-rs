use crate::{
    blockchain::Blockchain,
    crypto::{Hashable, Vrf},
    executable::Executable,
    keypair::Keypair,
    mempool,
    state::{GlobalState, Round},
    transaction::Transaction,
    database::Database
};
use log::*;
use serde::{Deserialize, Serialize};
use crate::blockchain::DATABASE_PATH_PREFIX;

#[derive(Deserialize, Serialize, Clone, Default, Debug)]

pub struct Header {
    pub height: u64,
    pub timestamp: u64,
    pub chain: u64,
    pub parent_hash: String,
    pub state_hash: String,
    pub uncle_root: String,
    pub proposer: String, // the publickey of the proposer
    pub transactions_merkle_root: String,
    pub header_payload: u8,
    pub proof: String,              // The vrf proof of the proposer as hex
    pub proposer_signature: String, // proposers signature
    pub validator_signatures: Vec<String>,
    pub vrf: String, // the hex encoded vrf proof used to sellect next rounds validating commitee and proposer
}

#[derive(Serialize, Clone, Default, Debug, Deserialize)]
pub struct Block {
    pub hash: String,
    pub header: Header,
    pub transactions: Vec<Transaction>,
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
    pub fn add_txn(&mut self, txn: Transaction) {
        self.transactions.push(txn);
    }
    pub fn get(hash: String) -> Result<Block, Box<dyn std::error::Error>> {
        Ok(Block::default())
    }
    pub fn form_vrf_tag(&mut self, keypair: &Keypair) -> Result<(), Box<dyn std::error::Error>> {
        if self.hash.len() == 0 {
            return Err("Hash must be set".into());
        } else if keypair.private_key.len() == 0 {
            return Err("Private key must be set".into());
        }
        let vrf = Vrf::generate(&keypair, self.hash.clone())?;
        self.header.vrf = vrf.proof;
        Ok(())
    }

    pub fn validate_vrf(&self, proposer: Keypair) -> Result<bool, Box<dyn std::error::Error>> {
        if self.header.vrf.len() == 0 {
            return Err("Vrf must be set".into());
        } else if self.hash.len() == 0 {
            return Err("Hash must be set".into());
        } else if proposer.public_key.len() == 0 {
            return Err("Proposer publickey must be set".into());
        }
        let vrf = Vrf::from_proof(&self.header.vrf)?;
        vrf.valid(proposer, &self.hash)
    }
}

impl Executable for Block {
    fn execute(&self, context: &String, globalState: &mut GlobalState) -> Result<String, Box<dyn std::error::Error>> {
        // Go through all the transactions and execute them
        let mut pre_applicate_state = globalState.clone();
        for txn in &self.transactions {
            let txn_result = txn.execute(context, &mut pre_applicate_state);
            if let Err(txn_error) = txn_result {
                return Err(txn_error.into());
            }
            let txn_result = txn_result.unwrap();
            let mut db_handle = Database::new();
            db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.header.chain))?;
            db_handle.set(&"transactions".to_owned(), &self.hash,&"1".to_string());
            log::debug!("txn_result: {}", txn_result);
        }
        // If we encountered no errors, we can apply the state
        *globalState = pre_applicate_state;
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
                            if self.header.state_hash != GlobalState::current()? {
                                return Err("Invalid state hash".into());
                            }
                            // TODO: Check uncle root (the merkle root of all the tips of the other chains)
                            // get the proposer by his address
                            let last_round = Round::get(self.header.height - 1, self.header.chain)?;
                            if self.header.proposer != last_round.proposer {
                                return Err(format!(
                                    "Invalid proposer, expected {}, got {}",
                                    last_round.proposer, self.header.proposer
                                )
                                .into());
                            }
                            // check the proposer signatuer is valid
                            let proposer = Keypair {
                                public_key: last_round.proposer,
                                private_key: String::default(),
                            };
                            match proposer.verify(&self.hash, &self.header.proposer_signature) {
                                Ok(valid) => {
                                    if !valid {
                                        return Err("Invalid proposer signature (signature)".into());
                                    }
                                }
                                Err(_) => {
                                    return Err("Invalid proposer signature (encoding)".into())
                                }
                            }
                            // now we check the VRF is valid
                            let vrf = Vrf::from_proof(&self.header.vrf)?;
                            match vrf.valid(proposer, &self.hash) {
                                Ok(valid) => {
                                    if !valid {
                                        return Err("Invalid proposer VRF (VRF)".into());
                                    }
                                }
                                Err(_) => return Err("Invalid proposer VRF (encoding)".into()),
                            }
                            if self.header.validator_signatures.len()
                                != last_round.validating_committee.len()
                            {
                                return Err("Not all validators accounted".into());
                            }
                            // now we check each validator signature
                            let mut valid_signers: Vec<String> = vec![];
                            for (index, v_sig) in
                                self.header.validator_signatures.iter().enumerate()
                            {
                                if v_sig.len() != 0 {
                                    // get the corrosponding publickey
                                    let validator = Keypair {
                                        public_key: last_round.validating_committee[index].clone(),
                                        private_key: String::default(),
                                    };
                                    if let Ok(valid) = validator.verify(&self.hash, v_sig) {
                                        if valid {
                                            valid_signers.push(validator.public_key)
                                        }
                                    }
                                }
                            }
                            // we have validated all the signatures, now check there is enough
                            if valid_signers.len()
                                < (1 / 3) * self.header.validator_signatures.len()
                            {
                                return Err(format!(
                                    "Not enough signatures, need {} got {}",
                                    (1 / 3) * self.header.validator_signatures.len(),
                                    valid_signers.len()
                                )
                                .into());
                            }
                            // check the transactions, start by getting them from mempool
                            for txn in &self.transactions {
                                match mempool::get_transaction(txn.hash.clone()) {
                                    Ok(tx) => {
                                        // validate the txn
                                        match tx.evalute() {
                                            Ok(_) => {
                                                debug!("Tx {} valid", txn.hash);
                                            }
                                            Err(error) => {
                                                // invalid transaction
                                                error!("Tx {} contained in block {} invalid, reason {}", txn.hash, self.hash, error);
                                                return Err(format!("Transaction {} included in block invalid, reason {}", txn.hash, error).into());
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        // TODO: ask peers for txn and look else where
                                        return Err(format!(
                                            "Failed to get txn {} from mempool, gave error {}",
                                            txn.hash, e
                                        )
                                        .into());
                                    }
                                }
                            }
                            return Ok(());
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
