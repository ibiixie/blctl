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

    fn get() -> Result<(), Box<dyn Error>> {
        println!("Backlight brightness level is 255");
        Ok(())
    }

    fn set(level: i32) -> Result<(), Box<dyn Error>> {
        println!("Setting backlight brightness level to {}", level);
        Ok(())
    }
}