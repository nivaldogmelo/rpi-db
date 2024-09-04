use std::{
    collections::BTreeMap,
    fs::File,
    io::{self, Read, Write},
};

use crate::domain::{DatabaseError, DatabaseTrait};

#[derive(Debug, Default)]
pub struct BincodeDB {
    data: BTreeMap<String, String>,
    file_path: String,
}

impl BincodeDB {
    pub fn new(file_path: &str) -> io::Result<Self> {
        let db = match BincodeDB::load(file_path) {
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
        let mut contents = vec![];
        file.read_to_end(&mut contents)?;

        let data: BTreeMap<String, String> = bincode::deserialize(&contents).unwrap();
        Ok(Self {
            data,
            file_path: file_path.to_string(),
        })
    }

    pub fn flush(&mut self) -> io::Result<()> {
        let contents = bincode::serialize(&self.data).unwrap();
        let mut file = File::create(&self.file_path)?;
        file.write_all(&contents)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl DatabaseTrait for BincodeDB {
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
