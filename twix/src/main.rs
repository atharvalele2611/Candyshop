use sysinfo::NetworkExt;

use sysinfo::{System, SystemExt};
use std::time::Duration;


fn main() {
    let mut s = System::new();

    loop {
        s.refresh_networks();

        let networks = s.networks();
        println!("==========================");
        for (interface_name, data) in networks {
            // println!("{:?}", data);
            println!(
                "[{}] in: {}, out: {}",
                interface_name,
                data.received(),
                data.transmitted(),
            );
        }

        std::thread::sleep(Duration::new(4, 0));
    }
}