use thiserror::Error;

pub trait HandlerTrait: Send + Sync {
    fn set(&mut self, key: String, value: String) -> Result<(), HandlerError>;
    fn get(&self, key: String) -> Result<String, HandlerError>;
    fn del(&mut self, key: String) -> Result<String, HandlerError>;
}

pub enum Handler {
    Set(String, String),
    Get(String),
    Del(String),
    Invalid,
}

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Key not found")]
    KeyNotFound,
    #[error("Command does not exists")]
    InvalidCommand,
    #[error("Unexpected error")]
    UnexpectedError,
}
