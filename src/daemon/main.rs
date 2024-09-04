#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::error::Error;

// use config::Config;

pub mod backlight;

use backlight::{Backlight, Sysfs};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting blctl daemon");

    // println!("Reading config");

    // let config_path = "/etc/blctl/config.toml";

    // let mut config_dirs = std::path::PathBuf::from(config_path);
    // config_dirs.pop();
    // std::fs::create_dir_all(config_dirs).unwrap();

    // let _config = Config::builder()
    //     .add_source(config::File::with_name(config_path).required(false))
    //     .build()
    //     .unwrap();

    println!("Initializing backlight interface");

    let backlight: Box<dyn Backlight> = Box::new(Sysfs::new()?);

    println!("Backlight initialized");

    backlight.set_brightness(10)?;

    Ok(())
}
