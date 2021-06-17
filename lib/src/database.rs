use std::collections::HashMap;

pub struct Database {
    dbs: HashMap<String, sled::Db>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            dbs: HashMap::new(),
        }
    }
    pub fn open(&mut self, path: &String) -> Result<sled::Db, Box<dyn std::error::Error>> {
        let db = sled::open(path)?;
        self.dbs.insert(path.to_owned(), db.clone());
        Ok(db)
    }
    pub fn get(&self, path: &String, key: &String) -> String {
        if let Some(db) = self.dbs.get(path) {
            // get the value
            if let Ok(value) = db.get(key) {
                if let Some(value_bytes) = value {
                    if let Ok(value_string) = String::from_utf8(value_bytes.to_vec()) {
                        return value_string;
                    }
                }
            }
        }
        String::default()
    }
    pub fn set(
        &self,
        path: &String,
        key: &String,
        value: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(db) = self.dbs.get(path) {
            db.insert(key.as_bytes(), value.as_bytes())?;
        }
        Err("Db not open".into())
    }
}
