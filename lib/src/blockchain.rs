use crate::database::Database;
use crate::block::Block;
use crate::block::Header;
use log::error;
use sled;
pub const DATABASE_PATH_PREFIX: &str = "./datadir/blockchain_db_"; // TODO: move to config
#[derive(Debug, Clone)]
pub struct Blockchain {
    index: u64,
}

impl Blockchain {
    pub fn new(index: u64) -> Self {
        let bc = Blockchain { index };
        bc
    }
    pub fn index(&self) -> u64 {
        self.index
    }
    pub fn tip(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut db_handle = Database::new(); // TODO: this is inefficent, have a global database handle pool OR make it a varible of the struct
        if let Ok(_) = db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.index)) {
            if let Some(tip) = db_handle.get(
                &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
                &"tip".to_string(),
            )? {
                return Ok(tip);
            } else {
                error!("Tip for blockchain {} not found in database", self.index);
                return Err("Cannot find tip in database".into());
            }
        } else {
            error!("Failed to open blockchain db for chain {}", self.index);
            return Err("Failed to open DB".into());
        }
    }

    pub fn to_string(&self) -> String {
        // TODO: is this really needed?
        format!("blockchain-{}", self.index())
    }
    pub fn from_string(as_string: String) -> Option<Self> {
        let split = as_string.split(':').collect::<Vec<&str>>();
        if split.len() == 1 {
            if let Ok(index) = split[0].parse() {
                return Some(Self { index });
            }
        }
        None
    }
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.index))?;
        db_handle.set(
            &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
            &String::from("save"),
            &self.to_string(),
        )?;
        Ok(())
    }
    pub fn load(index: u64) -> Result<Self, Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, index))?;
        if let Some(encoded) = db_handle.get(
            &format!("{}{}", DATABASE_PATH_PREFIX, index),
            &String::from("save"),
        )? {
            let bc = Blockchain::from_string(encoded).unwrap();
            return Ok(bc);
        } else {
            error!(
                "Faied to load blockchain {} from disk, not found in DB",
                index
            );
            return Err("Key not found in DB".into());
        }
    }
    pub fn list() -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        let mut list = Vec::new();
        let mut index = 0;
        loop {
            if let Ok(_) = db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, index)) {
                list.push(Self::load(index)?);
                index += 1;
            } else {
                break;
            }
        }
        Ok(list)
    }
    // save_block
    pub fn save_block(&self, block: &Block) -> Result<(), Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.index))?;
        db_handle.set(
            &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
            &block.hash(),
            &block.to_string(),
        )?;
        // now by height
        db_handle.set(
            &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
            &format!("height:{}", block.height()),
            &block.hash(),
        )?;
        Ok(())
    }
    // get_block_by_hash
    pub fn get_block_by_hash(
        hash: &str,
        chain: &u64
    ) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, chain))?;
        if let Some(encoded) = db_handle.get(
            &format!("{}{}", DATABASE_PATH_PREFIX, chain),
            &hash.to_string(),
        )? {
            let block = Block::from_string(encoded).unwrap();
            return Ok(Some(block));
        } else {
            return Ok(None);
        }
        drop(db_handle);
    }
    // get_block_by_height
    pub fn get_block_by_height(
        height: &u64,
        chain: &u64
    ) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, chain))?;
        if let Some(hash) = db_handle.get(
            &format!("{}{}", DATABASE_PATH_PREFIX, chain),
            &format!("height:{}", height),
        )? {
            if let Some(encoded) = db_handle.get(
                &format!("{}{}", DATABASE_PATH_PREFIX, chain),
                &hash.to_string(),
            )? {
                let block = Block::from_string(encoded).unwrap();
                return Ok(Some(block));
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
        drop(db_handle);
    }

    // create genesis chains
    pub fn create_genesis_blockchains() -> Result<(), Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        let mut blk_template = Block {
            hash: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
            header: Header {
                height: 0,
                timestamp: 0,
                chain: 0,
                parent_hash: "".to_owned(),
                state_hash: "".to_owned(),
                uncle_root: "".to_owned(),
                proposer: "".to_owned(), // the publickey of the proposer
                transactions_merkle_root: "".to_owned(),
                header_payload: 0,
                proof: "".to_owned(),              // The vrf proof of the proposer as hex
                proposer_signature: "".to_owned(), // proposers signature
                validator_signatures: vec!("".to_owned()),
                vrf: "".to_owned(), // the hex encoded vrf proof used to sellect next rounds validating commitee and proposer
            },
            // premine
            transactions: vec![],
        };
        // create 5 blockchains and add genesis block
        for i in 0..5 {
            let mut bc = Blockchain::new(i);
            let mut blk = blk_template.clone();
            blk.header.chain = i;
            // add the genesis block
            bc.save_block(&blk)?;
            // save the blockchain
            bc.save()?;
        }
        // drop the db handle
        drop(db_handle);
        Ok(())
    }
        // Uncle root = The root of a merkle tree composed of the top blocks (tips) of every chain
    pub fn generate_uncle_root(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.index))?;
        let mut uncle_root = MerkleTree::new();
        for i in 0..5 {
            if let Some(hash) = db_handle.get(
                &format!("{}{}", DATABASE_PATH_PREFIX, i),
                &String::from("tip"),
            )? {
                uncle_root.add_leaf(hash.as_bytes());
            }
        }
        Ok(uncle_root.get_root())
    }
    // set tip
    pub fn set_tip(&mut self, hash: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.index))?;
        db_handle.set(
            &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
            &String::from("tip"),
            &hash.to_string(),
        )?;
        Ok(())
    }

    // How would blocks sync, it will need to sync block 1 blockchain 1 then block 2 blockchain 1 then block 1 blockchain 2 etc
    // this is a very simple sync method as it will work with generate_uncle_root
}
