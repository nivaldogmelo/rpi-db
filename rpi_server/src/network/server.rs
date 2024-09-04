use std::{error::Error, sync::Arc};

use tokio::{net::TcpListener, sync::RwLock, task, try_join};

use crate::{
    domain::{DatabaseTrait, ReplTrait},
    repl::Repl,
};

pub type DatabaseArc = Arc<RwLock<dyn DatabaseTrait>>;

pub struct Server {
    repl_addr: String,
    db: DatabaseArc,
}

impl Server {
    pub fn build(repl_addr: &str, db: DatabaseArc) -> Self {
        Self {
            repl_addr: repl_addr.into(),
            db,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let repl_listener = TcpListener::bind(&self.repl_addr).await.unwrap();

        println!("REPL server running on {}", &self.repl_addr);

        let repl_db = Arc::clone(&self.db);
        let repl_handle = task::spawn(async move {
            loop {
                let (socket, _) = repl_listener.accept().await.unwrap();

                let db = repl_db.clone();

                tokio::spawn(async move {
                    let mut repl = Repl::new();
                    let _ = repl.run(socket, db).await;
                });
            }
        });

        try_join!(repl_handle)?;

        Ok(())
    }
}
