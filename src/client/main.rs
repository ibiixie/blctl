#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;

mod blctl;
use blctl::{Blctl, CliArgs};

mod ipc;

fn main() -> Result<(), String> {
    println!("Client init v1.0.0-alpha");

    let args = CliArgs::parse();
    Blctl::new(args.verbose).run(args.request())
}
