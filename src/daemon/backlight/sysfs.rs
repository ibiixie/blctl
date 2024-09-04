use crate::backlight::Backlight;

use std::{
    error::Error,
    fs::OpenOptions,
    io::{Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

const CLASS_PATH: &str = "/sys/class/backlight/";

pub struct Sysfs {
    device_path: PathBuf,
}

impl Backlight for Sysfs {
    fn new() -> Result<Self, Box<dyn Error>> {
        let device_path = Path::new(CLASS_PATH);

        if !device_path.exists() {
            return Err("no backlight class found in sysfs".into());
        }

        // Retrieve the first device in the backlight class
        if let Some(path) = (std::fs::read_dir(device_path)?).next() {
            let device_name = path?.file_name().into_string().unwrap();
            let device_path = PathBuf::from(CLASS_PATH).join(device_name);

            Ok(Self { device_path })
        } else {
            Err("no backlight devices found in sysfs".into())
        }
    }

    fn set_brightness(&self, level: i32) -> Result<(), Box<dyn Error>> {
        let mut level = level;
        let max_brightness = self.max_brightness()?;

        // Clamp to maximum allowed value as defined by the backlight device
        if level > max_brightness {
            level = max_brightness;
        }

        self.device_write("brightness", level.to_string().as_bytes())
            .unwrap();

        Ok(())
    }

    fn brightness(&self) -> Result<i32, Box<dyn Error>> {
        self.device_read::<i32>("brightness")
    }
}

impl Sysfs {
    fn device_write<'a, T>(&self, file_name: &str, value: T) -> Result<(), Box<dyn Error>>
    where
        T: Into<&'a [u8]>,
    {
        let path = self.device_path.clone().join(file_name);
        let mut file = OpenOptions::new().truncate(true).write(true).open(path)?;

        file.write_all(value.into())?;

        Ok(())
    }

    fn device_read<T>(&self, file_name: &str) -> Result<T, Box<dyn Error>>
    where
        T: FromStr + std::fmt::Debug,
    {
        let path = self.device_path.clone().join(file_name);
        let mut file = OpenOptions::new().read(true).open(path)?;

        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        Ok(buffer
            .parse()
            .map_err(|_| {
                Err::<T, Box<dyn Error>>("failed to parse data read from backlight device".into())
            })
            .unwrap())
    }

    fn max_brightness(&self) -> Result<i32, Box<dyn Error>> {
        self.device_read("max_brightness")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_backlight() -> Result<(), Box<dyn Error>> {
        Sysfs::new()?;
        Ok(())
    }

    #[test]
    fn read_brightness() -> Result<(), Box<dyn Error>> {
        let backlight = Sysfs::new()?;
        backlight.device_read::<i32>("brightness")?;
        Ok(())
    }

    #[test]
    fn read_max_brightness() -> Result<(), Box<dyn Error>> {
        let backlight = Sysfs::new()?;
        backlight.device_read::<i32>("max_brightness")?;
        Ok(())
    }

    #[test]
    fn write_brightness() -> Result<(), Box<dyn Error>> {
        let backlight = Sysfs::new()?;

        backlight.device_write("brightness", 0.to_string().as_bytes())?;

        let max = backlight.device_read::<i32>("max_brightness")?;
        backlight.device_write("brightness", max.to_string().as_bytes())?;

        Ok(())
    }

    #[test]
    fn set_brightness() -> Result<(), Box<dyn Error>> {
        let backlight = Sysfs::new()?;

        backlight.set_brightness(0)?;
        assert_eq!(backlight.brightness()?, 0);

        Ok(())
    }
}
