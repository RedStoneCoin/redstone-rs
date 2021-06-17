use crate::{
    account::Account, block::Block, blockchain::Blockchain, crypto::Hashable, database::Database,
};

pub struct GlobalState {
    accounts: Vec<Account>,
    tips: Vec<Block>,
    chains: Vec<Blockchain>,
}

impl Hashable for GlobalState {
    fn bytes(&self) -> Vec<u8> {
        let mut out = vec![];
        for acc in &self.accounts {
            if acc.smart_contract {
                // TODO: include the state hash of the smart contract
            } else {
                out.extend(format!("{}{}.", acc.address, acc.balance).bytes())
            }
        }
        for tip in &self.tips {
            out.extend(hex::decode(&tip.hash).unwrap());
        }
        for chain in &self.chains {
            out.extend(chain.index().to_string().bytes());
        }
        out
    }
}

impl GlobalState {
    pub fn current() -> Result<String, Box<dyn std::error::Error>> {
        let mut db = Database::new();
        db.open(&"globalstate".to_string())?;
        Ok(db.get(&"globalstate".to_string(), &"hash".to_string()))
    }
    pub fn set_current(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::new();
        db.open(&"globalstate".to_string())?;
        db.set(
            &"globalstate".to_string(),
            &"hash".to_string(),
            &self.hash_item(),
        )
    }
}
