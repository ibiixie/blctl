use clap::Subcommand;
use serde::{Deserialize, Serialize};

pub const IPC_SOCKET_FILE_PATH: &str = "/tmp/blctl/blctld.sock";

#[derive(Subcommand, Deserialize, Serialize, Clone, Copy, Debug)]
pub enum Request {
    /// Set backlight brightness to the specified level
    Set {
        /// Desired brightness level (between 0 and 100 inclusive unless '--from-raw' is passed)
        level: i32,

        /// Interpret the specified brightness level as a raw backlight value
        #[arg(long, short)]
        from_raw: bool,

        /// Print the resulting brightness level as a raw backlight value
        #[arg(long, short)]
        to_raw: bool,
    },

    /// Increase backlight brightness by the specified amount
    Increase {
        /// Desired brightness level to increase by (between 0 and 100 inclusive unless '--from-raw' is passed)
        amount: i32,

        /// Interpret the specified brightness amount as a raw backlight value
        #[arg(short, long)]
        from_raw: bool,

        /// Print the resulting brightness level as a raw backlight value
        #[arg(short, long)]
        to_raw: bool,
    },

    /// Decrease backlight brightness by the specified amount
    Decrease {
        /// Desired brightness level to decrease by (between 0 and 100 inclusive unless '--from-raw' is passed)
        amount: i32,

        /// Interpret the specified brightness amount as a raw backlight value
        #[arg(long, short)]
        from_raw: bool,

        /// Print the resulting brightness level as a raw backlight value
        #[arg(long, short)]
        to_raw: bool,
    },

    /// Return the current backlight brightness level
    Get {
        /// Print the current brightness level as a raw backlight value
        #[arg(long, short)]
        to_raw: bool,
    },

    /// Return the raw maximum backlight brightness level
    GetMax,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Response {
    Success { level: i32, raw: bool },
    Failure { reason: String },
}
