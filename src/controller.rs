use crate::kernelinterface;

// Paths are relative to the backlight device directory
const BACKLIGHT_BRIGHTNESS_FILEPATH: &'static str = "brightness";
const BACKLIGHT_MAX_BRIGHTNESS_FILEPATH: &'static str = "max_brightness";

pub struct BacklightController {
    kernel_brightness_fp: String,
    kernel_max_brightness_fp: String
}

impl BacklightController {
    pub fn new(interface_path: &String) -> Self {
        let interface_path = std::path::Path::new(interface_path);
        let kernel_brightness_fp = interface_path.join(std::path::Path::new(BACKLIGHT_BRIGHTNESS_FILEPATH));
        let kernel_max_brightness_fp = interface_path.join(std::path::Path::new(BACKLIGHT_MAX_BRIGHTNESS_FILEPATH));

        Self {
            kernel_brightness_fp: kernel_brightness_fp.to_str().unwrap().to_string(),
            kernel_max_brightness_fp: kernel_max_brightness_fp.to_str().unwrap().to_string()
        }
    }

    pub fn increase_brightness(&self, amount: f32) {
        let current = self.get_brightness();
        let new = current + amount;

        self.set_brightness(new);
    }

    pub fn decrease_brightness(&self, amount: f32) {
        let current = self.get_brightness();
        let new = current - amount;

        self.set_brightness(new);
    }

    pub fn set_brightness(&self, mut value: f32) {
        if value > 1f32 {
            value = 1f32;
        } else if value < 0f32 {
            value = 0f32
        }

        let value_abs = self.percent_to_abs(value);

        kernelinterface::write(&self.kernel_brightness_fp, value_abs.to_string());
    }

    pub fn get_brightness(&self) -> f32 {
        let brightness_abs = kernelinterface::read(&self.kernel_brightness_fp)
            .trim()
            .parse::<u32>()
            .expect(format!(
                "failed to parse kernel interface ({}) data to u32",
                &self.kernel_brightness_fp
            ).as_str());

        let abs_percent = self.abs_to_percent(brightness_abs);

        self.round_to_decimal(abs_percent, 2)
    }

    /// Somewhat unreliably rounds an `f32` value to the specified amount of
    /// decimals. I should proably fix this at some point.
    fn round_to_decimal(&self, value: f32, decimal: u32) -> f32 {
        let power = i32::pow(10, decimal) as f32;
        f32::round(value * power) / power
    }

    /// Converts a backlight brightness percentage to the equivalent
    /// absolute backlight brightness value.
    fn percent_to_abs(&self, value: f32) -> u32 {
        let max_abs = kernelinterface::read(&self.kernel_max_brightness_fp)
            .trim()
            .parse::<u32>()
            .expect(format!(
                "failed to parse kernel interface ({}) data to u32",
                &self.kernel_max_brightness_fp
            ).as_str());

        let absolute = value * max_abs as f32;

        absolute.round() as u32
    }

    /// Converts an absolute brightness percentage to the equivalent
    /// backlight brightness percentage.
    fn abs_to_percent(&self, value: u32) -> f32 {
        let max_abs = kernelinterface::read(&self.kernel_max_brightness_fp)
            .trim()
            .parse::<u32>()
            .expect(format!(
                "failed to parse kernel interface ({}) data to u32",
                &self.kernel_max_brightness_fp
            ).as_str());

        value as f32 / max_abs as f32
    }
}
