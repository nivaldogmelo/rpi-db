use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Write},
};

use crate::domain::{DatabaseError, DatabaseTrait};

#[derive(Debug, Default)]
pub struct JsonDB {
    data: BTreeMap<String, String>,
    file_path: String,
}

impl JsonDB {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let db = match JsonDB::load(file_path) {
            Ok(db) => db,
            Err(_) => Self {
                data: BTreeMap::new(),
                file_path: file_path.to_string(),
            },
        };

        Ok(db)
    }

    pub fn load(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let data: BTreeMap<String, String> = serde_json::from_str(&contents)?;
        Ok(Self {
            data,
            file_path: file_path.to_string(),
        })
    }

    pub fn flush(&mut self) -> io::Result<()> {
        let contents = serde_json::to_string(&self.data)?;
        let mut file = File::create(&self.file_path)?;
        file.write_all(contents.as_bytes())
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl DatabaseTrait for JsonDB {
    fn insert(&mut self, key: String, value: String) -> Result<(), DatabaseError> {
        self.data.insert(key, value);
        let _ = self.flush();
        Ok(())
    }

    fn search(&self, key: String) -> Result<String, DatabaseError> {
        match self.data.get(&key) {
            Some(value) => Ok(value.clone()),
            None => Err(DatabaseError::KeyNotFound),
        }
    }

    fn delete(&mut self, key: String) -> Result<String, DatabaseError> {
        match self.data.remove(&key) {
            Some(value) => Ok(value.clone()),
            None => Err(DatabaseError::KeyNotFound),
        }
    }
}
