use std::error::Error;

pub mod sysfs;

pub use sysfs::Sysfs;

pub trait Backlight {
    /// Create a backlight using the specified backlight interface path.
    ///
    /// # Errors
    /// N/A
    fn new() -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;

    /// Sets the brightness to the specified amount in percent from 0 to 100.
    ///
    /// # Errors
    /// N/A
    fn set_brightness(&self, level: i32) -> Result<(), Box<dyn Error>>;

    /// Returns the backlight brightness in percent.
    ///
    /// # Errors
    /// N/A
    fn brightness(&self) -> Result<i32, Box<dyn Error>>;

    /// Returns the maximum backlight brightness.
    /// 
    /// # Notes
    /// The value returned is dependent on the backlight device driver.
    /// Different vendors use different maximum values for their backlight.
    /// 
    /// # Errors
    /// N/A
    fn brightness_max(&self) -> Result<i32, Box<dyn Error>>;
}
