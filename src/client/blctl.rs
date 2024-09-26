use std::path::Path;

use clap::Parser;

use crate::ipc::IpcClient;
use blctl_shared::*;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
#[command(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Request>,

    /// Print detailed debugging output to stdout
    #[arg(long, short)]
    pub verbose: bool,
}

impl CliArgs {
    #[must_use]
    pub fn request(&self) -> Request {
        self.command.clone().unwrap()
    }
}

pub struct Blctl {
    verbose: bool,
}

impl Blctl {
    pub fn new(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Run the specified command/request
    pub fn run(&self, request: Request) -> Result<(), String> {
        let mut ipc_client = IpcClient::new(Path::new(IPC_SOCKET_FILE_PATH));
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
