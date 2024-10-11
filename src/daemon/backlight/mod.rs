use std::error::Error;

pub mod sysfs;

pub use sysfs::Sysfs;

pub trait Backlight {
    /// Create a new backlight implementation instance.
    fn new() -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;

    /// Set the backlight brightness to the specified level.
    fn set_brightness(&self, level: i32) -> Result<i32, Box<dyn Error>>;

    /// Get the current brightness level.
    fn brightness(&self) -> Result<i32, Box<dyn Error>>;

    /// Get the maxmimum supported brightness level.
    fn brightness_max(&self) -> Result<i32, Box<dyn Error>>;
}
