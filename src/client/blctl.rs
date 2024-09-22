use std::error::Error;

use clap::Parser;

use blctl_shared::Command;

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
}

pub struct Blctl {

}

impl Blctl {
    pub fn new() -> Self {
        Self {

        }
    }

    pub fn set(&self, level: i32, raw: bool) -> i32 {
        println!("Setting backlight brightness to {level} (raw: {raw})");
        0
    }

    pub fn increase(&self, amount: i32, raw: bool) -> i32 {
        println!("Increasing backlight brightness by {amount} (raw: {raw})");
        0
    }

    pub fn decrease(&self, amount: i32, raw: bool) -> i32 {
        println!("Decreasing nacklight brightness by {amount} (raw: {raw})");
        0
    }

    pub fn get(&self, raw: bool) -> i32 {
        println!("Current brightness is 0 (raw: {raw})");
    }

    pub fn get_max(&self) -> i32 {
        println!("Maximum supported brightness is 255");
    }

    pub fn store(&self) -> i32 {
        println!("Stored current brightness");
    }

    pub fn restore(&self) -> i32 {
        println!("Stored brightness was restored");
    }
}