use std::io::{Read, Write};

use clap::Parser;

use blctl_shared::{Request, Response};

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
    pub fn run(&self, request: Request) -> Response {
        let result = match request {
            Request::Set { level, raw } => self.set(level, raw),
            Request::Increase { amount, raw } => self.increase(amount, raw),
            Request::Decrease { amount, raw } => self.decrease(amount, raw),
            Request::Get { raw } => self.get(raw),
            Request::GetMax => self.get_max(),
            Request::Store => self.store(),
            Request::Restore => self.restore(),
        };

        let mut ipc_stream = std::os::unix::net::UnixStream::connect("/tmp/blctld.sock")
            .expect("failed to open IPC socket - is the blctl daemon running?");
        
        let request_data = bincode::serialize(&request).unwrap();
        ipc_stream.write(request_data.as_slice()).unwrap();
        
        let mut response_data: Vec<u8>;
        ipc_stream.read_to_end(&mut response_data).unwrap();
        let response = bincode::deserialize::<Response>(response_data.as_slice()).unwrap();

        dbg!(response);

        result
    }

    pub fn set(&self, level: i32, raw: bool) -> Response {
        if self.verbose {
            println!("Setting backlight brightness to {level} (raw: {raw})");
        }

        Response::Success {
            level: 0,
            raw: false,
        }
    }

    pub fn increase(&self, amount: i32, raw: bool) -> Response {
        println!("Increasing backlight brightness by {amount} (raw: {raw})");
        Response::Success {
            level: 0,
            raw: false,
        }
    }

    pub fn decrease(&self, amount: i32, raw: bool) -> Response {
        println!("Decreasing nacklight brightness by {amount} (raw: {raw})");
        Response::Success {
            level: 0,
            raw: false,
        }
    }

    pub fn get(&self, raw: bool) -> Response {
        println!("Current brightness is 0 (raw: {raw})");
        Response::Success {
            level: 0,
            raw: false,
        }
    }

    pub fn get_max(&self) -> Response {
        println!("Maximum supported brightness is 255");
        Response::Success {
            level: 0,
            raw: false,
        }
    }

    pub fn store(&self) -> Response {
        println!("Stored current brightness");
        Response::Success {
            level: 0,
            raw: false,
        }
    }

    pub fn restore(&self) -> Response {
        println!("Stored brightness was restored");
        Response::Success {
            level: 0,
            raw: false,
        }
    }
}
