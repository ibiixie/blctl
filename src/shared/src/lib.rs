use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Subcommand, Deserialize, Serialize, Clone, Debug)]
pub enum Request {
    /// Set backlight brightness to the specified level
    Set {
        /// Desired brightness level between 0 and 100 inclusive
        level: i32,

        /// Interpret the brightness level to be a raw value defined by the
        /// backlight device driver as opposed to a value mapped between
        /// 0 and 100 inclusive
        #[arg(long)]
        raw: bool,
    },

    /// Increase backlight brightness by the specified amount
    Increase {
        /// Desired brightness level to increase by between 0 and 100 inclusive
        amount: i32,

        /// Interpret the brightness level to be a raw value defined by the
        /// backlight device driver as opposed to a value mapped between
        /// 0 and 100 inclusive
        #[arg(long)]
        raw: bool,
    },

    /// Decrease backlight brightness by the specified amount
    Decrease {
        /// Desired brightness level to decrease by between 0 and 100 inclusive
        amount: i32,

        /// Interpret the brightness level to be a raw value defined by the
        /// backlight device driver as opposed to a value mapped between
        /// 0 and 100 inclusive
        #[arg(long)]
        raw: bool,
    },

    /// Return the current backlight brightness level
    Get {
        /// Return the brightness level as a raw value defined by the
        /// backlight device driver as opposed to a value mapped between
        /// 0 and 100 inclusive
        #[arg(long)]
        raw: bool,
    },

    /// Return the maximum supported backlight brightness level as defined
    /// by the backlight device driver
    GetMax,

    /// Store the current backlight brightness level on disk
    Store,

    /// Set backlight brightness to the previously stored value
    Restore,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum Response {
    Success { level: i32, raw: bool },
    Failure { reason: String },
}
