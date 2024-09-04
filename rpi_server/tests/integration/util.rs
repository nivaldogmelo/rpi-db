use std::sync::Arc;

use rpi_db::{network::Server, storage::JsonDB};
use tokio::sync::RwLock;

pub struct TestDatabase {}

impl TestDatabase {
    pub async fn new() {
        let db = Arc::new(RwLock::new(JsonDB::new("./test.db").unwrap()));

        let repl_addr = "127.0.0.1:8081";

        let mut server = Server::build(repl_addr, db);

        server.start().await.unwrap();
    }
}
