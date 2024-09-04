use thiserror::Error;

use super::Handler;

pub trait ReplTrait {
    fn new() -> Self;
    fn parse(&self, input: &str) -> Result<Handler, ReplError>;
}

#[derive(Debug, Error)]
pub enum ReplError {
    #[error("Command does not exists")]
    InvalidCommand,
    #[error("Command with invalid syntax")]
    InvalidSyntax(String),
    #[error("Unexpected error")]
    UnexpectedError,
}
