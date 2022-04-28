pub mod database;
pub mod snickers_commands;

use std::sync::Arc;

use clap::Parser;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::{Mutex, RwLock},
};

use crate::database::Database;

// use skittles::{self, log};
#[derive(Parser, Debug)]
#[clap(name = "Snickers", version = "1.0")]
struct Args {
    #[clap(long, default_value_t = 8080)]
    port: i32,

    #[clap(short, default_value = "localhost")]
    hostname: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let hostname = args.hostname;
    let port = args.port.to_string();
    let listener = TcpListener::bind(format!("{}:{}", hostname, port))
        .await
        .unwrap();

    // tokio::spawn(log(listener1));
    // to allow multiple clients conncet to server
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        // let socket1 = Arc::new(RwLock::new(socket));
        // let sc = Arc::clone(&socket1);
        // tokio::spawn(log(sc));
        let mut db = Database::new();

        tokio::spawn(async move {
            // let mut socket1 = socket1.write().await;
            let (read, mut writer) = socket.split();
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
                        None => {
                            let response = String::from("UNKNOWN COMMAND\n");
                            writer.write_all(response.as_bytes()).await.unwrap();
                        }
                    }
                }
                line.clear();
            }
        });
    }
}
