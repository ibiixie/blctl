#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::error::Error;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Set { level: i32, raw: bool },
    Increase { amount: i32, raw: bool },
    Decrease { amount: i32, raw: bool },
    Get { raw: bool },
    GetMax,
    Store,
    Restore,
}

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    #[must_use]
    pub fn command(&self) -> Command {
        self.command.clone().unwrap()
    }

    fn get() -> Result<(), Box<dyn Error>> {
        println!("Backlight brightness level is 255");
        Ok(())
    }

    fn set(level: i32) -> Result<(), Box<dyn Error>> {
        println!("Setting backlight brightness level to {}", level);
        Ok(())
    }
}

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