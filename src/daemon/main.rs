#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::path::Path;

mod backlight;
mod blctld;

use blctld::Daemon;

fn main() {
    println!("Daemon init");

    let daemon = Daemon::new(Path::new("/tmp/blctld.sock"));
    daemon.listen();
}
