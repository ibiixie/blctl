mod kernelinterface;
mod controller;

use std::{
    error::Error,
    future::pending
};

use controller::BacklightController;

use zbus::{ConnectionBuilder, dbus_interface};

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

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting blctl daemon");
    
    println!("Creating backlight controller");

    let bl_controller = BacklightController {
         kernel_brightness_fp: 
             "/sys/class/backlight/amdgpu_bl0/brightness".to_string(),
         kernel_max_brightness_fp: 
             "/sys/class/backlight/amdgpu_bl0/max_brightness".to_string()
    };

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
