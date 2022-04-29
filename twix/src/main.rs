mod rules;

use std::sync::Arc;
use std::io::{self, Error, ErrorKind};

use std::time::Duration;

use mars::Mars;
use skittles::SkittlesClient;
use clap::Parser;
use sysinfo::{System, SystemExt, NetworkExt};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, MutexGuard};

use crate::rules::Rules;

#[derive(Parser, Debug)]
#[clap(name="Twix", version = "1.0")]
struct Args {
    #[clap(long, default_value = "TWIX-1")]
    name: String,

    #[clap(short, long, default_value_t = 10)]
    interval: u64,

    #[clap(short, long, default_value_t = 4545)]
    port: u16,

    #[clap(short, long, default_value_t = 1.0)]
    ram: f32,

    #[clap(short, long, default_value_t = 1.0)]
    memory: f32,

    #[clap(short, long, default_value_t = 1)]
    network: usize,

    #[clap(long)]
    log: bool,

    #[clap(long)]
    log_ip: String
}

#[tokio::main(flavor = "current_thread")] // single threaded
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let log = SkittlesClient::new(args.name, args.log_ip, args.log);

    let mut mars = Mars::new(log.clone());
    mars.add_topic("ram,memory,network").await;
    let mars = Arc::new(Mutex::new(mars));

    let mars_sys = Arc::clone(&mars);

    let system = tokio::spawn(async move {
        use tokio::net::TcpListener;

        let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await
                                    .expect("Could not start TCP Server");

        loop {
            let (socket, _addr) = listener.accept().await.expect("Error while accepting socket");
            {
                let guard = mars_sys.lock().await;

                let _ = process(guard, socket).await;    
            }
        }
    });

    let mars_req = Arc::clone(&mars);
    let req_log = log.clone();
    let req = tokio::spawn(async move {
        let mut s = System::new();
        let r = Rules::new(args.ram, args.memory, args.network);
        loop {
            s.refresh_cpu();
            s.refresh_memory();
            s.refresh_networks();

            let ram = s.available_memory();
            let mut network = 0;
            let _ = s.networks().into_iter().map(|f| network += f.1.transmitted());

            let (b1, b2, b3) = r.check(0.0, ram as f32, network as usize);

            {
                let mars = mars_req.lock().await;

                if b1 {
                    req_log.log("RAM Limit Crossed").await;
                    mars.send_topic_message("ram", b"Limit Crossed").await;
                }

                if b2 {
                    req_log.log("Storage Limit Crossed").await;
                    mars.send_topic_message("memory", b"Limit Crossed").await;
                }

                if b3 {
                    req_log.log("Bandwidth Limit Crossed").await;
                    mars.send_topic_message("network", b"Limit Crossed").await;
                }
            }

            tokio::time::sleep(Duration::new(args.interval, 0)).await;
        }
    });

    let _ = tokio::join!(system, req);

    Ok(())
}

async fn process(mut mars: MutexGuard<'_, Mars>, s: TcpStream) -> io::Result<()> {
    s.readable().await?;

    let mut buffer = [0_u8; 512];
    let n = s.try_read(&mut buffer)?;

    match String::from_utf8(buffer[0..n].to_vec()) {
        Ok(string) => {
            mars.add_subscriber(s, &string).await;
        },
        Err(_e) => return io::Result::Err(Error::new(ErrorKind::InvalidData, "Invalid Data")),
    }

    Ok(())
}