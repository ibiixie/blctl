mod kernelinterface;
mod controller;

use std::{
    error::Error,
    future::pending
};

use controller::BacklightController;

use zbus::{ConnectionBuilder, dbus_interface};

use config::Config;

struct DBusMessageHandler {
    controller: BacklightController
}

#[dbus_interface(name = "me.xela.blctl1")]
impl DBusMessageHandler {
    async fn increase(&self, amount:f32) {
        self.controller.increase_brightness(amount); 
    }

    async fn decrease(&self, amount: f32) {
        self.controller.decrease_brightness(amount);
    }

    async fn set(&self, value: f32) {
        self.controller.set_brightness(value);
    }

    async fn get(&self) -> f32 {
       self.controller.get_brightness() 
    }
}

// Returns an auto-detected backlight interface path or the overriden 
// path if one is set in the config file.
fn get_backlight_interface_path(conf: &Config) -> String {
    let interface_path = if let Ok(interface_override) = conf.get::<String>("interface_override") {
        println!("Backlight interface override enabled! Using '{interface_override}' as backlight interface");
        interface_override
    } else {
        let interface_directory = std::path::Path::new("/sys/class/backlight/");
        let mut interface_iter = interface_directory.read_dir().unwrap();
        let interface_first = interface_iter.nth(0).unwrap().unwrap();

        let interface_path = interface_first.path();

        println!("Detected backlight interface {:?}", interface_path);

        interface_path.to_string_lossy().to_string()
    };

    interface_path
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting blctl daemon");
    
    println!("Reading config");

    let conf = Config::builder()
        .add_source(config::File::with_name("/etc/blctl/config.toml"))
        .build()
        .unwrap();

    println!("Creating backlight controller");

    let bl_interface_path = get_backlight_interface_path(&conf);
    let bl_controller = BacklightController::new(&bl_interface_path);
        
    println!("Creating D-Bus message handler");

    let dbus_handler = DBusMessageHandler {
        controller: bl_controller
    };

    println!("Building D-Bus connection");

    let _conn = ConnectionBuilder::system()?
        .name("me.xela.blctl")?
        .serve_at("/me/xela/blctl", dbus_handler)?
        .build()
        .await?;

    println!("Awaiting D-Bus messages");

    pending::<()>().await;

    Ok(())
}
