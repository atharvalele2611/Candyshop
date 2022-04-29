pub mod database;
pub mod snickers_commands;

use std::sync::Arc;

use clap::Parser;
use mars::Mars;
use skittles::SkittlesClient;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Mutex, RwLock},
};

use crate::database::Database;

#[derive(Parser, Debug)]
#[clap(name = "Snickers", version = "1.0")]
struct Args {
    #[clap(long, default_value_t = 8080)]
    port: i32,

    #[clap(long, default_value = "SNICKERS-1")]
    name: String,

    #[clap(long)]
    log: bool,

    #[clap(long)]
    log_ip: String,

    #[clap(long)]
    master_ip: Option<String>,

    #[clap(long)]
    is_master: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let hostname = "localhost";
    let port = args.port.to_string();
    let master_ip = args.master_ip;
    let is_master = args.is_master;
    let listener = TcpListener::bind(format!("{}:{}", hostname, port))
        .await
        .unwrap();
    let mars = Arc::new(Mutex::new(Mars::new()));
    let db = Arc::new(RwLock::new(Database::new()));

    if master_ip.is_some() && !is_master {
        let rip = master_ip.clone().unwrap();
        // let tl = TcpListener::bind(rip).await.unwrap();
        // let (mut s, _a) = tl.accept().await.unwrap();

        let mut stream = TcpStream::connect(rip).await.unwrap();
        let _ = stream
            .write_all(b"hash,trie,lists,strings,sets,server")
            .await;

        let db = Arc::clone(&db);
        let mars = Arc::clone(&mars);

        tokio::spawn(async move {
            loop {
                let _ = stream.readable().await;

                let mut buf = [0_u8; 512];
                let n = stream.try_read(&mut buf);

                if n.is_err() {
                    continue;
                }
                let line = match String::from_utf8(buf[0..n.unwrap()].to_vec()) {
                    Ok(string) => string,
                    Err(_e) => continue,
                };
                let mut input = Vec::<&str>::new();
                for arg in line.split_ascii_whitespace() {
                    input.push(arg);
                }
                if !input.is_empty() && input.len() >= 2 {
                    let command = input[0];
                    let database_key = input[1];
                    let values = &input[2..];
                    let cmd = snickers_commands::lookup(command).await;
                    match cmd {
                        Some(cmd) => {
                            let _ = cmd
                                .execute(
                                    &mut db.write().await,
                                    command,
                                    database_key,
                                    values,
                                    is_master,
                                    mars.lock().await,
                                )
                                .await;
                        }
                        None => (),
                    }
                }
            }
        });
    }

    if is_master {
        let mars = Arc::clone(&mars);
        {
            mars.lock()
                .await
                .add_topic("hash,trie,lists,strings,sets,server")
                .await
        }
        let rip = master_ip.unwrap();
        let tl = TcpListener::bind(rip).await.unwrap();
        tokio::spawn(async move {
            loop {
                let (s, _a) = tl.accept().await.unwrap();
                let _ = s.readable().await;
                let mut buf = [0_u8; 512];
                let n = s.try_read(&mut buf).unwrap();
                match String::from_utf8(buf[0..n].to_vec()) {
                    Ok(string) => {
                        mars.lock().await.add_subscriber(s, &string).await;
                    }
                    Err(_e) => (),
                }
            }
        });
    }

    let log = SkittlesClient::new(args.name, args.log_ip, args.log);
    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();

        let log = log.clone();
        let db = Arc::clone(&db);
        let mars = Arc::clone(&mars);
        tokio::spawn(async move {
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
                    let cmd = snickers_commands::lookup(command).await;
                    log.log(command).await;
                    match cmd {
                        Some(cmd) => {
                            let res = cmd
                                .execute(
                                    &mut db.write().await,
                                    command,
                                    database_key,
                                    values,
                                    is_master,
                                    mars.lock().await,
                                )
                                .await;

                            if res.is_ok() {
                                let ok = res.unwrap();
                                writer.write_all(ok.as_bytes()).await.unwrap();
                                let ok = ok.replace('\n', " ");
                                log.log(&ok).await;
                            } else {
                                let err = res.unwrap_err();
                                writer.write_all(err.as_bytes()).await.unwrap();
                                let err = err.replace('\n', " ");
                                log.log(&err).await;
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
pub mod tests;
