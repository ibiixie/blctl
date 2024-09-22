#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::error::Error;

use clap::Parser;

mod blctl;
use blctl::{Blctl, CliArgs};

use blctl_shared::Response;

fn main() -> Result<(), String> {
    let args = CliArgs::parse();

    // Run the cli command and make the corresponding request to the daemon
    return match Blctl::new(args.verbose).run(args.request()) {
        Response::Success { level, raw } => {
            println!("{level}");
            Ok(())
        }
        Response::Failure { reason } => {
            println!("{reason}");
            Err(reason)
        }
    };
}
