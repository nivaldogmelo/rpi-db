use rpi_db::Database;

#[tokio::main]
async fn main() {
    let repl_address = "127.0.0.1:8081";
    let db = Database::build(repl_address);

    let _ = db.expect("Failed to start database").start().await;
}
