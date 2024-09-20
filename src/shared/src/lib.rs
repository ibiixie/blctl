use clap::Subcommand;

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