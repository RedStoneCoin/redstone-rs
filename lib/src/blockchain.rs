use crate::database::Database;
use crate::block::Block;
use crate::block::Header;
use crate::transaction::Transaction;
use crate::crypto::hash;
use log::error;
use sled;
pub const DATABASE_PATH_PREFIX: &str = "./datadir/blockchain_db_"; // TODO: move to config
#[derive(Debug, Clone, Copy)]
pub struct Blockchain {
    index: u64,
}
// TODO: Move 0..5 to db so we can do for more then 5 chains
// more then 5 chains is planed for main net
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
            &format!("height:{}", block.height()),
            &block.hash,
        )?;
        // now hash
        db_handle.set(
            &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
            &format!("hash:{}", block.hash),
            &block.to_string(),
        )?;
        // save blockchain
        self.save()?;
        Ok(())
    }

    // get_block_by_height
    pub fn get_block_by_height(
        self,
        height: &u64,
    ) -> Result<Option<Block>, Box<dyn std::error::Error>> {
        let chain = self.index();
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
        // get_block_by_hash
        pub fn get_block_by_hash(
            self,
            hash: &str,
        ) -> Result<Block, Box<dyn std::error::Error>> {
            let mut db_handle = Database::new();
            let chain = self.index();
            db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, chain))?;
            if let Some(encoded) = db_handle.get(
                &format!("{}{}", DATABASE_PATH_PREFIX, chain),
                &format!("hash:{}", hash),
            )? {
                let block = Block::from_string(encoded).unwrap();
                return Ok(block);
            } else {
                return Err("Block not found".into());
            }
            drop(db_handle);
        }
    // set tip
    pub fn set_tip(&mut self, hash: &str,height: &u64) -> Result<(), Box<dyn std::error::Error>> {
        let chain = self.index();
        let mut db_handle = Database::new();
        db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, chain))?;
        db_handle.set(
            &format!("{}{}", DATABASE_PATH_PREFIX, chain),
            &String::from("tip"),
            &hash.to_string(),
        )?;
        db_handle.set(
            &format!("{}{}", DATABASE_PATH_PREFIX, chain),
            &String::from("tip_height"),
            &height.to_string(),
        )?;
        self.save()?;

        Ok(())
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
                // uncle_root_height: // array of the hights that make the uncle root
                uncle_root_height: vec![0,0,0,0,0],
            },
            // trannsaction type craete chain.
            transactions: vec![
                Transaction {
                    hash: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
                    sender: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
                    reciver: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
                    amount: 0,
                    type_flag: 0,
                    nonce: 0,
                    payload: "".to_owned(), // Hex encoded payload
                    signature: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
                }
            ],
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
        // generate tips
        for i in 0..5 {
            // now set_tip
            let mut bc = Blockchain::new(i);
            bc.set_tip(&"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),&0)?;
        }
        
        // drop the db handle
        drop(db_handle);
        Ok(())

    }
    // Uncle root = The root of a merkle tree composed of the top blocks (tips) of every chain
    // uncle_root_hight: // array of the hights that make the uncle root




        // get height
        pub fn get_height(&self) -> Result<u64, Box<dyn std::error::Error>> {
            let mut db_handle = Database::new();
            db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.index))?;
            if let Some(height) = db_handle.get(
                &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
                &String::from("tip_height"),
            )? {
                return Ok(height.parse::<u64>().unwrap());
            } else {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Block not found",
                )));
            }
            drop(db_handle);
        }
    // fn generate_uncle root
    // function generates uncle root and uncle root height 
    pub fn generate_uncle_root(&self) -> Result<(String,Vec<u64>), Box<dyn std::error::Error>> {
        // uncle root is hash of the top blocks of each chain
        // uncle root height is the array of block heights that make the uncle root
        // so we can verify the uncle root
        let mut db_handle = Database::new();
        let mut uncle_root = vec![];
        let mut uncle_root_height = vec![];
        for i in 0..5 {
            let mut bc = Blockchain::new(i);
            let block = bc.get_block_by_hash(&bc.tip()?)?;
            uncle_root.push(block.hash.clone());
            uncle_root_height.push(block.header.height.clone());
        }
        // uncle root to bytes then hash
        let mut uncle_root_bytes = vec![];
        for i in 0..uncle_root.len() {
            uncle_root_bytes.append(&mut uncle_root[i].as_bytes().to_vec());
        }
        let uncle_root_hash = hash(uncle_root_bytes);
        Ok((uncle_root_hash,uncle_root_height))
    }
    // verify uncle root
    pub fn verify_uncle_root(&self,uncle_root: &str,uncle_root_height: &Vec<u64>) -> Result<bool, Box<dyn std::error::Error>> {
        // uncle root is hash of the top blocks of each chain
        // uncle root height is the array of block heights that make the uncle root
        // so we can verify the uncle root
        let mut db_handle = Database::new();
        let mut uncle_root_hash = vec![];
        for i in 0..5 {
            let mut bc = Blockchain::new(i);
            let block = bc.get_block_by_hash(&bc.tip()?)?;
            uncle_root_hash.push(block.hash.clone());
        }
        // uncle root to bytes then hash
        let mut uncle_root_bytes = vec![];
        for i in 0..uncle_root_hash.len() {
            uncle_root_bytes.append(&mut uncle_root_hash[i].as_bytes().to_vec());
        }
        let uncle_root_hash = hash(uncle_root_bytes);
        if uncle_root_hash == uncle_root {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }





    // How would blocks sync, it will need to sync block 1 blockchain 1 then block 2 blockchain 1 then block 1 blockchain 2 etc
    // this is a very simple sync method as it will work with generate_uncle_root
}

// test for uncle root
#[test]
fn test_uncle_root() {
    /// remopve datadir
    let _ = std::fs::remove_dir_all("datadir");
    // crate genesis chains and blocks
    // start
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
            // uncle_root_height: // array of the hights that make the uncle root
            uncle_root_height: vec![0,0,0,0,0],
        },
        // trannsaction type craete chain.
        transactions: vec![
            Transaction {
                hash: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
                sender: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
                reciver: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
                amount: 0,
                type_flag: 0,
                nonce: 0,
                payload: "".to_owned(), // Hex encoded payload
                signature: "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),
            }
        ],
    };
    // create 5 blockchains and add genesis block
    for i in 0..5 {
        let mut bc = Blockchain::new(i);
        let mut blk = blk_template.clone();
        blk.header.chain = i;
        // add the genesis block
        bc.save_block(&blk);
        // save the blockchain
        bc.save();
    }
    // generate tips
    for i in 0..5 {
        // now set_tip
        let mut bc = Blockchain::new(i);
        bc.set_tip(&"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_owned(),&0);
    }

    // drop the db handle
    drop(db_handle);
    //end
    // TEST uncle root
    let mut bc = Blockchain::new(0);
    let (uncle_root,uncle_root_height) = bc.generate_uncle_root().unwrap();
    let verify = bc.verify_uncle_root(&uncle_root,&uncle_root_height).unwrap();
    assert_eq!(verify,true);
    // print the uncle root
    eprintln!("uncle root: {}",uncle_root);
    eprintln!("uncle root height: {:?}",uncle_root_height);
    // delete the datadir
    let _ = std::fs::remove_dir_all("datadir");
}
// what is commadn to run test in /lib/src/blockchain.rs: cargo test --lib -- --nocapture
