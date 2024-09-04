use thiserror::Error;

pub trait DatabaseTrait: Send + Sync {
    fn insert(&mut self, key: String, value: String) -> Result<(), DatabaseError>;
    fn search(&self, key: String) -> Result<String, DatabaseError>;
    fn delete(&mut self, key: String) -> Result<String, DatabaseError>;
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Key not found")]
    KeyNotFound,
    #[error("Unexpected error")]
    UnexpectedError,
}
