use crate::backlight::{Backlight, Sysfs};

struct Daemon {
    // backlight: Box<dyn Backlight>
}

impl Daemon {
    fn new() -> Self {
        Self {
            // backlight: Box::new(Sysfs::new());
        }
    }
}
