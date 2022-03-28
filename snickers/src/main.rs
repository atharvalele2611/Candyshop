use std::{collections::HashMap, str::FromStr};

use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

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
        let mut hash_map = HashMap::<String, HashMap<String, String>>::new();
        let (mut socket, addr) = listener.accept().await.unwrap();
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
                let db = input[1];
                let values = &input[2..];
                println!("command {:?} db {:?} ", command, db);
                println!("values {:?}", values);

                if command.to_ascii_lowercase().eq("hmset") {
                    if values.is_empty() || values.len() % 2 != 0 {
                        println!("error");
                    } else {
                        if !hash_map.contains_key(db) {
                            hash_map.insert(db.to_string(), HashMap::<String, String>::new());
                        }
                        let mut key_idx = 0 as usize;
                        let hm_db = hash_map.get_mut(&db.to_string()).unwrap();
                        while key_idx < values.len() - 1 {
                            let key = values[key_idx];
                            let val = values[key_idx + 1];
                            hm_db.insert(key.to_string(), val.to_string());
                            key_idx = key_idx + 2;
                        }
                        writer.write_all("OK\n".as_bytes()).await.unwrap();
                    }
                }
                if command.to_ascii_lowercase().eq("hmget") {
                    if values.is_empty() {
                        println!("error");
                    } else {
                        if !hash_map.contains_key(db) {
                            println!("error");
                        } else {
                            let hm_db = hash_map.get_mut(&db.to_string()).unwrap();
                            for key in values {
                                match hm_db.get_mut(&key.to_string()) {
                                    Some(val) => {
                                        val.push('\n');
                                        writer.write(val.as_bytes()).await.unwrap();
                                    }
                                    None => {
                                        let nil = String::from_str("(nil)\n").unwrap();
                                        writer.write(nil.as_bytes()).await.unwrap();
                                    }
                                }
                            }
                        }
                    }
                }

                println!("keys in org hash map{:?}", hash_map.keys());
                let mut hm_test = hash_map.get_mut(&db.to_string()).unwrap();
                for (k, v) in hm_test {
                    println!("test k {:?} val {:?}", k, v);
                }
                // writer.write_all(line.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
