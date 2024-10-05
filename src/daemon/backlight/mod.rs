use std::error::Error;

pub mod sysfs;

pub use sysfs::Sysfs;

pub trait Backlight {
    fn new() -> Result<Self, Box<dyn Error>>
    where
        Self: std::marker::Sized;

    fn set_brightness(&self, level: i32) -> Result<i32, Box<dyn Error>>;

    fn brightness(&self) -> Result<i32, Box<dyn Error>>;

    fn brightness_max(&self) -> Result<i32, Box<dyn Error>>;
}
