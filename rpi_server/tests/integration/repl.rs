use serial_test::serial;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
};

use crate::util::TestDatabase;

#[tokio::test]
#[serial]
async fn invalid_command() {
    tokio::spawn(TestDatabase::new());

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line).await.unwrap();
    line.clear();

    writer.write_all(b"Lorem\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> InvalidCommand");
}

#[tokio::test]
#[serial]
async fn set_command() {
    tokio::spawn(TestDatabase::new());

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line).await.unwrap();
    line.clear();

    writer.write_all(b"SET foo\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(
        response_line,
        ">> InvalidSyntax for SET\nUsage: SET <key> <value>"
    );

    writer.write_all(b"SET foo bar\n").await.unwrap();
    writer.flush().await.unwrap();
    line.clear();

    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> Ok");
}

#[tokio::test]
#[serial]
async fn get_command() {
    tokio::spawn(TestDatabase::new());

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line).await.unwrap();
    line.clear();

    writer.write_all(b"SET foo bar\n").await.unwrap();
    writer.flush().await.unwrap();
    reader.read_line(&mut line).await.unwrap();

    line.clear();
    writer.write_all(b"GET foo bar\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> InvalidSyntax for GET\nUsage: GET <key>");

    line.clear();
    writer.write_all(b"GET bar\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> KeyNotFound");

    line.clear();
    writer.write_all(b"GET foo\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> bar");
}

#[tokio::test]
#[serial]
async fn del_command() {
    tokio::spawn(TestDatabase::new());

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let mut stream = TcpStream::connect("127.0.0.1:8081").await.unwrap();
    let (reader, mut writer) = stream.split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    reader.read_line(&mut line).await.unwrap();
    line.clear();

    writer.write_all(b"SET foo bar\n").await.unwrap();
    writer.flush().await.unwrap();
    reader.read_line(&mut line).await.unwrap();

    line.clear();
    writer.write_all(b"DEL foo bar\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> InvalidSyntax for DEL\nUsage: DEL <key>");

    line.clear();
    writer.write_all(b"DEL bar\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> KeyNotFound");

    line.clear();
    writer.write_all(b"DEL foo\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> bar");
    line.clear();

    writer.write_all(b"GET foo\n").await.unwrap();
    writer.flush().await.unwrap();

    reader.read_line(&mut line).await.unwrap();
    let response_line = line.trim();
    assert_eq!(response_line, ">> KeyNotFound");
}
