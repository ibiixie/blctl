#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::error::Error;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    Get,
    Set { level: i32 },
    Increase { amount: i32 },
    Decrease { amount: i32 },
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
        Command::Get => {
            get();
        }
        Command::Set { level } => {
            set(level);
        }
        Command::Increase { amount } => {
            increase(amount);
        }
        Command::Decrease { amount } => {
            decrease(amount);
        }
    }

    Ok(())
}

fn get() {
    println!("0");
}

fn set(level: i32) {
    println!("{level}");
}

fn increase(amount: i32) {
    println!("{amount}");
}

fn decrease(amount: i32) {
    println!("{amount}");
}
