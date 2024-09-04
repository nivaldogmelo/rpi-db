use std::{error::Error, sync::Arc};

use network::Server;
use storage::JsonDB;
use tokio::sync::RwLock;

pub mod core;
pub mod domain;
pub mod network;
pub mod repl;
pub mod storage;

pub struct Database {
    server: Server,
}

impl Database {
    pub fn build(repl_addr: &str) -> Result<Database, Box<dyn Error>> {
	let db = Arc::new(RwLock::new(JsonDB::new("./data.json").unwrap()));

	let server = Server::build(repl_addr, db);

	Ok(Self { server })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
	self.server.start().await
    }
}
