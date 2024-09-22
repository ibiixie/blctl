#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::error::Error;

use clap::Parser;

mod blctl;

use blctl::Cli;
use blctl_shared::Command;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    

    match args.command() {
        Command::Set { level, raw } => {
            
        }
        Command::Increase { amount, raw } => {

        }
        Command::Decrease { amount, raw } => {

        }
        Command::Get { raw } => {
            
        }
        Command::GetMax => {
            
        }
        Command::Store => {
            
        }
        Command::Restore => {
            
        }
    }

    Ok(())
}