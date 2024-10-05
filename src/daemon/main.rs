#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

use std::path::Path;

mod backlight;
mod blctld;

use blctl_shared::IPC_SOCKET_FILE_PATH;
use blctld::Daemon;

fn main() {
    println!("Starting blctl daemon v{}", env!("CARGO_PKG_VERSION"));

    let daemon = Daemon::new(Path::new(IPC_SOCKET_FILE_PATH));
    daemon.listen();
}
