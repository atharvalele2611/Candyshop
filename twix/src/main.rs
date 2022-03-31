#![allow(unused)]

mod rules;

use std::io::{self, Error, ErrorKind};
use std::os::unix::thread;
use std::time::Duration;

use mars::Mars;

use clap::Parser;
use sysinfo::{System, SystemExt, ProcessorExt, NetworkExt};
use tokio::net::TcpStream;

#[derive(Parser, Debug)]
#[clap(name="Twix", version = "1.0")]
struct Args {
    #[clap(short, long)]
    interval: i32,

    #[clap(short, long)]
    port: u16,

    #[clap(short, long)]
    rules: String,

    #[clap(long, default_value_t = 99.9)]
    ram: f32,

    #[clap(long, default_value_t = 99.9)]
    memory: f32,

    #[clap(long, default_value_t = 99.9)]
    network: f32
}

#[tokio::main(flavor = "current_thread")] // single threaded
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut mars = Mars::new();

    mars.add_topic("ram,memory,network");

    let system = tokio::spawn(async move {
        use tokio::net::TcpListener;

        let listener = TcpListener::bind(format!("127.0.0.1:{}", args.port)).await
                                    .expect("Could not start TCP Server");

        loop {
            let (mut socket, _addr) = listener.accept().await.expect("Error while accepting socket");

            process(&mut mars, socket).await;
        }
    });

    let req = tokio::spawn(async move {
        loop {
            std::thread::sleep(Duration::new(args.interval as u64, 0));
        }
    });

    tokio::join!(system, req);

    Ok(())

    // let mut s = System::new();

    // loop {
    //     s.refresh_networks();
    //     s.refresh_components();
    //     s.refresh_components_list();
    //     s.refresh_cpu();
    //     s.refresh_networks();
    //     s.refresh_networks_list();

    //     println!("--------------------------");
    //     println!("free_memory {:?}", (s.free_memory() as f64) / 1000000.00);
    //     println!("used_memory {:?}", (s.used_memory() as f64) / 1000000.00);
    //     println!(
    //         "available_memory {:?}",
    //         (s.available_memory() as f64) / 1000000.00
    //     );

    //     println!("processors {:?}", s.processors());
    //     for p in s.processors() {
    //         println!("processors {:?}", p.cpu_usage());
    //     }

    //     let networks = s.networks();
    //     for (interface_name, data) in networks {
    //         // println!("{:?}", data);
    //         println!(
    //             "[{}] in: {}, out: {}",
    //             interface_name,
    //             data.received(),
    //             data.transmitted(),
    //         );
    //     }
    //     println!("==========================");
    //     std::thread::sleep(Duration::new(4, 0));
    // }
}


async fn process(mars: &mut Mars, s: TcpStream) -> io::Result<()> {
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