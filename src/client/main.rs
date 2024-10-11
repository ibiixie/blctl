#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use clap::Parser;

mod blctl;
use blctl::{Blctl, CliArgs};

mod ipc;

fn main() -> Result<(), String> {
    let args = CliArgs::parse();
    Blctl::new().run(args.request())
}
