pub mod database;
pub mod snickers_commands;

use std::sync::Arc;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::{Mutex, RwLock},
};

use crate::database::Database;

use skittles::{self, log};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    // tokio::spawn(log(listener1));
    // to allow multiple clients conncet to server
    loop {
        let (socket, _addr) = listener.accept().await.unwrap();
        let socket1 = Arc::new(RwLock::new(socket));
        let sc = Arc::clone(&socket1);
        tokio::spawn(log(sc));
        let mut db = Database::new();

        tokio::spawn(async move {
            let mut socket1 = socket1.write().await;
            let (read, mut writer) = socket1.split();
            let mut reader = BufReader::new(read);
            loop {
                let mut line = String::new();
                let bytes_r = reader.read_line(&mut line).await.unwrap();
                if bytes_r == 0 {
                    break;
                }
                let mut input = Vec::<&str>::new();
                for arg in line.split_ascii_whitespace() {
                    input.push(arg);
                }
                if !input.is_empty() && input.len() >= 2 {
                    let command = input[0];
                    let database_key = input[1];
                    let values = &input[2..];
                    let cmd = snickers_commands::lookup(command);
                    match cmd {
                        Some(cmd) => {
                            let res = cmd.execute(&mut db, database_key, values);

                            if res.is_ok() {
                                writer.write_all(res.unwrap().as_bytes()).await.unwrap();
                            } else {
                                writer.write_all(res.unwrap_err().as_bytes()).await.unwrap();
                            }
                        }
                        None => (),
                    }
                }

                // line.clear();
            }
        });
    }
}
