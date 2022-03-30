pub mod database;
pub mod redis_commands;

use std::{collections::HashMap, str::FromStr};

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

use crate::database::Database;

// tests
/*
hmset drivers p1 Verstappen p2 Leclerc p3 Sainz p4 Perez
hmget drivers p1 p2 p3 p4

hmset constructors p1 Ferrari p2 Mercedes p3 Redbull p4 Alpine
hmget constructors p1 p2 p3 p4
*/

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    // to allow multiple clients conncet to server

    loop {
        let (mut socket, _addr) = listener.accept().await.unwrap();
        let mut db = Database::new();
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
                let command = input[0];
                let database_key = input[1];
                let values = &input[2..];
                println!("command {:?} db {:?} ", command, database_key);
                println!("values {:?}", values);

                let cmd = redis_commands::lookup(command);
                match cmd {
                    Some(cmd) => {
                        let res = cmd.execute(&mut db, database_key, values);
                        println!("{:?}", res);
                        //write res in writer
                    }
                    None => (),
                }
                // writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
