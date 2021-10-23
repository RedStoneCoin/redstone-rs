use crate::{
    account::Account,
    block::Block,
    blockchain::Blockchain,
    crypto::{Hashable, Vrf},
    database::Database,
    validators::{choose_next_proposer, form_validating_commitee, Validator},
};

pub struct Round {
    pub chain: u64,
    pub round: u64,
    pub proposer: String,
    pub validating_committee: Vec<String>,
}
impl Round {
    pub fn form_from_block(block: &Block) -> Result<Round, Box<dyn std::error::Error>> {
        let mut round = Round {
            chain: block.header.chain,
            round: block.header.height,
            proposer: String::default(),
            validating_committee: vec![],
        };
        // get the last block
        let prev_block = Block::get(block.header.parent_hash.clone())?;
        // choose the proposer
        let proposer = choose_next_proposer(prev_block.clone())?;
        round.proposer = proposer.public_key.clone();

        // choose size of validating commitee
        let online = Validator::get_online()?;
        if online.len() < 3 {
            // there is no other nodes, use the proposer as the validating commitee
            round.validating_committee = vec![proposer.public_key];
        } else {
            // now choose the validating commitee
            // TODO: dynmaic commitee size
            let committee = form_validating_commitee(prev_block, proposer, 2);
            round.validating_committee = committee?
                .iter()
                .map(|v| v.public_key.clone())
                .collect::<Vec<String>>();
        }
        Ok(round)
    }
    pub fn from_string(as_string: String) -> Result<Round, Box<dyn std::error::Error>> {
        let decoded_string = String::from_utf8(hex::decode(as_string)?)?;
        let split = decoded_string.split(".").collect::<Vec<&str>>();
        let mut out = Round {
            chain: split[0].parse()?,
            round: split[1].parse()?,
            proposer: split[2].to_string(),
            validating_committee: vec![],
        };
        for v in split[3].split("-") {
            if v.len() != 0 {
                out.validating_committee.push(v.to_string())
            }
        }
        Ok(out)
    }
    pub fn to_string(&self) -> String {
        let mut vc = String::default();
        for v in &self.validating_committee {
            vc += &format!("{}-", v);
        }
        hex::encode(format!(
            "{}.{}.{}.{}",
            self.chain, self.round, self.proposer, vc
        ))
    }
    pub fn get(round: u64, chain: u64) -> Result<Round, Box<dyn std::error::Error>> {
        let mut db = Database::new();
        db.open(&format!("rounds-{}", chain))?;
        let encoded = db.get(&format!("rounds-{}", chain), &round.to_string());
        Round::from_string(encoded)
    }
    pub fn set(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db = Database::new();
        db.open(&format!("rounds-{}", self.chain))?;
        let encoded = self.to_string();
        db.set(
            &format!("rounds-{}", self.chain),
            &self.round.to_string(),
            &encoded,
        )
    }
}

#[derive(Clone)]
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
