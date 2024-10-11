use std::path::Path;

use clap::Parser;

use crate::ipc::Client;
use blctl_shared::{Request, Response, IPC_SOCKET_FILE_PATH};

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Request>,
}

impl CliArgs {
    #[must_use]
    pub fn request(&self) -> Request {
        self.command.unwrap()
    }
}

pub struct Blctl;

impl Blctl {
    pub fn new() -> Self {
        Self {}
    }

    /// Run the specified command/request
    #[allow(clippy::unused_self)]
    pub fn run(&self, request: Request) -> Result<(), String> {
        let mut ipc_client = Client::new(Path::new(IPC_SOCKET_FILE_PATH));
        let response = ipc_client.request(request);

        match response {
            Response::Success { level, .. } => {
                println!("{level}");
                Ok(())
            }
            Response::Failure { reason } => {
                eprintln!("Request failed: {reason}");
                Err(reason)
            }
        }
    }
}
