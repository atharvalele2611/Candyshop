use sysinfo::{System, SystemExt};
use lazy_static::lazy_static;

lazy_static! {
    static ref SYSTEM: System = System::new();
}

