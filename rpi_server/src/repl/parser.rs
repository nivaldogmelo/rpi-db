use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt},
    net::TcpStream,
};

use crate::{
    domain::{Handler, HandlerError, HandlerTrait, ReplError, ReplTrait},
    network::DatabaseArc,
};

pub struct Repl {}

impl ReplTrait for Repl {
    fn new() -> Self {
        Repl {}
    }

    fn parse(&self, input: &str) -> Result<Handler, ReplError> {
        // Parse input
        let parts: Vec<&str> = input.split_whitespace().collect();
        let parts = parts.as_slice();
        match parts {
            ["SET", key, value] => Ok(Handler::Set(key.to_string(), value.to_string())),
            ["GET", key] => Ok(Handler::Get(key.to_string())),
            ["DEL", key] => Ok(Handler::Del(key.to_string())),
            [cmd, ..] => match *cmd {
                "SET" => Err(ReplError::InvalidSyntax("SET".to_string())),
                "GET" => Err(ReplError::InvalidSyntax("GET".to_string())),
                "DEL" => Err(ReplError::InvalidSyntax("DEL".to_string())),
                _ => Err(ReplError::InvalidCommand),
            },
            _ => Err(ReplError::InvalidCommand),
        }
    }
}

impl Repl {
    pub async fn run(&mut self, socket: TcpStream, db: DatabaseArc) -> Result<(), ReplError> {
        let (reader, mut writer) = io::split(socket);
        let mut reader = io::BufReader::new(reader);
        let mut line = String::new();

        writer
            .write_all(b"Welcome to rpi REPL. Type 'exit' to quit.\n")
            .await
            .unwrap();
        writer.write_all(b">> ").await.unwrap();

        loop {
            line.clear();

            let bytes_read = reader.read_line(&mut line).await.unwrap();

            if bytes_read == 0 {
                continue;
            }

            if line.trim() == "exit" {
                break;
            }

            let handle = match self.parse(line.trim()) {
                Ok(h) => h,
                Err(e) => {
                    let response = repl_into_response(e);
                    writer.write_all(response.as_bytes()).await.unwrap();
                    writer.write_all(b"\n>> ").await.unwrap();
                    continue;
                }
            };

            let response = self.execute(handle, db.clone()).await;
            writer.write_all(response.as_bytes()).await.unwrap();
            writer.write_all(b"\n>> ").await.unwrap();
        }

        writer.write_all(b"Goodbye!\n").await.unwrap();

        Ok(())
    }

    async fn execute(&mut self, handle: Handler, db: DatabaseArc) -> String {
        match handle {
            Handler::Set(key, value) => {
                let mut db = db.write().await;

                match db.set(key, value) {
                    Ok(_) => "Ok".to_owned(),
                    Err(e) => handler_into_response(e),
                }
            }
            Handler::Get(key) => {
                let db = db.read().await;

                match db.get(key) {
                    Ok(v) => v,
                    Err(e) => handler_into_response(e),
                }
            }
            Handler::Del(key) => {
                let mut db = db.write().await;

                match db.del(key) {
                    Ok(v) => v,
                    Err(e) => handler_into_response(e),
                }
            }
            Handler::Invalid => todo!(),
        }
    }
}

fn handler_into_response(e: HandlerError) -> String {
    match e {
        HandlerError::KeyNotFound => "KeyNotFound".to_owned(),
        HandlerError::InvalidCommand => "InvalidCommand".to_owned(),
        HandlerError::UnexpectedError => "UnexpectedError".to_owned(),
    }
}

fn repl_into_response(e: ReplError) -> String {
    match e {
        ReplError::InvalidCommand => "InvalidCommand".to_owned(),
        ReplError::InvalidSyntax(cmd) => match cmd.as_str() {
            "SET" => "InvalidSyntax for SET\nUsage: SET <key> <value>".to_owned(),
            "GET" => "InvalidSyntax for GET\nUsage: GET <key>".to_owned(),
            "DEL" => "InvalidSyntax for DEL\nUsage: DEL <key>".to_owned(),
            _ => "InvalidCommand".to_owned(),
        },
        ReplError::UnexpectedError => "UnexpectedError".to_owned(),
    }
}
