#![allow(unused)]

mod rules;

use std::sync::Arc;
use std::io::{self, Error, ErrorKind};
use std::os::unix::thread;
use std::time::Duration;

use mars::Mars;

use clap::Parser;
use sysinfo::{System, SystemExt, ProcessorExt, NetworkExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, MutexGuard};

use crate::rules::Rules;

#[derive(Parser, Debug)]
#[clap(name="Twix", version = "1.0")]
struct Args {
    #[clap(short, long)]
    interval: u64,

    #[clap(short, long)]
    port: u16,

    #[clap(short, long)]
    rules: String,

    #[clap(long, default_value_t = 99.9)]
    ram: f32,

    #[clap(long, default_value_t = 99.9)]
    memory: f32,

    #[clap(long, default_value_t = 10240)]
    network: usize
}

#[tokio::main(flavor = "current_thread")] // single threaded
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut mars = Arc::new(Mutex::new(Mars::new()));

    mars.lock().await.add_topic("ram,memory,network");

    let system = tokio::spawn(async move {
        use tokio::net::TcpListener;

        let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await
                                    .expect("Could not start TCP Server");

        loop {
            let (mut socket, _addr) = listener.accept().await.expect("Error while accepting socket");
            {
                let guard = mars.lock().await;

                process(guard, socket).await;    
            }
            
        }
    });

    let req = tokio::spawn(async move {
        let mut s = System::new();
        let mut r = Rules::new(args.ram, args.memory, args.network);
        loop {
            s.refresh_cpu();
            s.refresh_memory();
            s.refresh_networks();

            let ram = s.available_memory();
            let mut network = 0;
            s.networks().into_iter().map(|f| network += f.1.transmitted());

            let (b1, b2, b3) = r.check(0.0, ram as f32, network as usize);

            {
                let mars = mars.lock().await;

                if b1 {
                    mars.send_topic_message("ram", b"Limit Crossed");
                }

                if b2 {
                    mars.send_topic_message("memory", b"Limit Crossed");
                }

                if b3 {
                    mars.send_topic_message("network", b"Limit Crossed");
                }
            }

            tokio::time::sleep(Duration::new(args.interval, 0)).await;
        }
    });

    tokio::join!(system, req);

    Ok(())
}

async fn process(mut mars: MutexGuard<'_, Mars>, s: TcpStream) -> io::Result<()> {
    s.readable().await?;

    let mut buffer = [0_u8; 512];
    let n = s.try_read(&mut buffer)?;

    match String::from_utf8(buffer[0..n].to_vec()) {
        Ok(string) => {
            mars.add_subscriber(s, &string);
        },
        Err(e) => return io::Result::Err(Error::new(ErrorKind::InvalidData, "Invalid Data")),
    }

    Ok(())
}