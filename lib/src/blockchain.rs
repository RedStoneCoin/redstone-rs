use crate::database::Database;
use sled;
pub const DATABASE_PATH_PREFIX: &str = "./datadir/blockchain_db_";
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
    pub fn tip(&self) -> String {
        let mut db_handle = Database::new();
        if let Ok(_) = db_handle.open(&format!("{}{}", DATABASE_PATH_PREFIX, self.index)) {
            return db_handle.get(
                &format!("{}{}", DATABASE_PATH_PREFIX, self.index),
                &"tip".to_string(),
            );
        }
        String::default()
    }
    pub fn to_string(&self) -> String {
        format!("{}:", self.index())
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
        let encoded = db_handle.get(
            &format!("{}{}", DATABASE_PATH_PREFIX, index),
            &String::from("save"),
        );
        let bc = Blockchain::from_string(encoded).unwrap();
        Ok(bc)
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
