use crate::database::Database;
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
    pub fn to_string(&self) -> String { // TODO: is this really needed?
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
        } else  {
            error!("Faied to load blockchain {} from disk, not found in DB", index);
            return Err("Key not found in DB".into())
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
    pub fn test_chains() -> Vec<Self> {
        let mut chains = Vec::new();
        for i in 0..10 {
            let mut bc = Blockchain::new(i);
            bc.save().unwrap();
            chains.push(bc);
        }
        chains
    }
}
