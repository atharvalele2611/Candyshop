use sysinfo::NetworkExt;

use std::time::Duration;
use sysinfo::{System, SystemExt};

fn main() {
    let mut s = System::new();

    loop {
        s.refresh_networks();
        s.refresh_components();
        s.refresh_components_list();
        s.refresh_cpu();
        s.refresh_networks();
        s.refresh_networks_list();

        println!("--------------------------");
        println!("free_memory {:?}", (s.free_memory() as f64) / 1000000.00);
        println!("used_memory {:?}", (s.used_memory() as f64) / 1000000.00);
        println!(
            "available_memory {:?}",
            (s.available_memory() as f64) / 1000000.00
        );

        println!("processors {:?}", s.processors());
        for p in s.processors() {
            println!("processors {:?}", p.cpu_usage());
        }

        let networks = s.networks();
        for (interface_name, data) in networks {
            // println!("{:?}", data);
            println!(
                "[{}] in: {}, out: {}",
                interface_name,
                data.received(),
                data.transmitted(),
            );
        }
        println!("==========================");
        std::thread::sleep(Duration::new(4, 0));
    }
}
