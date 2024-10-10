use std::error::Error;
use std::fs::Permissions;
use std::io::{Read, Write};
use std::path::Path;

use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};

use crate::backlight::{Backlight, Sysfs};

use blctl_shared::{Request, Response};

pub struct Daemon {
    listener: UnixListener,
    backlight: Box<dyn Backlight>,
}

impl Daemon {
    /// Create and initialize a new daemon socket instance.
    pub fn new(path: &Path) -> Self {
        let backlight = Box::new(Sysfs::new().expect("unable to create sysfs backlight interface"));

        if path.exists() {
            println!("Removing previous socket");
            std::fs::remove_file(path).unwrap();
        } else {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        }

        let listener = UnixListener::bind(path).unwrap();

        println!("Bound to socket");

        std::fs::set_permissions(path, Permissions::from_mode(0o666)).unwrap();

        println!("Set socket file permissions");

        Self {
            listener,
            backlight,
        }
    }

    /// Blockingly listen for incoming client connections and handle them.
    pub fn listen(&self) {
        println!("Awaiting socket connections");

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Client connected to socket");
                    match self.handle_client(&stream) {
                        Ok(()) => println!("Client request completed"),
                        Err(err) => eprintln!("Failed to handle client request: {err}"),
                    }
                }
                Err(err) => {
                    println!("Failed to accept incoming connection: {err}");
                    continue;
                }
            }
        }
    }

    /// Handle the client.
    fn handle_client(&self, client_stream: &UnixStream) -> Result<(), Box<dyn Error>> {
        println!("Handling client");

        let request = self.read_request(client_stream)?;

        let response = match self.handle_request(request) {
            Ok((level, raw)) => Response::Success { level, raw },
            Err(err) => Response::Failure {
                reason: err.to_string(),
            },
        };

        self.send_response(client_stream, &response)?;

        Ok(())
    }

    /// Read a request from the given client stream.
    #[allow(clippy::unused_self)]
    fn read_request(&self, mut client_stream: &UnixStream) -> Result<Request, Box<dyn Error>> {
        println!("Reading request");

        let mut request_size = [0u8; std::mem::size_of::<usize>()];
        client_stream.read_exact(&mut request_size)?;

        let request_size = usize::from_ne_bytes(request_size);
        println!("Request data size is {request_size} bytes");

        let mut request_data = vec![0u8; request_size];
        client_stream.read_exact(&mut request_data)?;

        let request = bincode::deserialize::<Request>(&request_data)?;

        println!("Request received");
        dbg!(&request);

        Ok(request)
    }

    /// Send the specified response to the given client stream.
    #[allow(clippy::unused_self)]
    fn send_response(
        &self,
        mut client_stream: &UnixStream,
        response: &Response,
    ) -> Result<(), Box<dyn Error>> {
        println!("Sending response");

        let response_data = bincode::serialize(&response)?;
        println!("Response data size is {} bytes", response_data.len());

        client_stream.write_all(&response_data.len().to_ne_bytes())?;
        client_stream.write_all(&response_data)?;

        println!("Response sent");
        dbg!(&response);

        Ok(())
    }

    fn handle_request(&self, request: Request) -> Result<(i32, bool), Box<dyn Error>> {
        println!("Handling request");

        match request {
            Request::Set {
                mut level,
                from_raw,
                to_raw,
            } => {
                if !from_raw {
                    level = self.map_percent_to_raw(level)?;
                }

                let mut result = self.backlight.set_brightness(level)?;

                if !to_raw {
                    result = self.map_raw_to_percent(result)?;
                }

                Ok((result, to_raw))
            }
            Request::Increase {
                mut amount,
                from_raw,
                to_raw,
            } => {
                let brightness = self.backlight.brightness()?;

                if !from_raw {
                    amount = self.map_percent_to_raw(amount)?;
                }

                let mut result = self.backlight.set_brightness(brightness + amount)?;

                if !to_raw {
                    result = self.map_raw_to_percent(result)?;
                }

                Ok((result, to_raw))
            }
            Request::Decrease {
                mut amount,
                from_raw,
                to_raw,
            } => {
                let brightness = self.backlight.brightness()?;

                if !from_raw {
                    amount = self.map_percent_to_raw(amount)?;
                }

                let mut result = self.backlight.set_brightness(brightness - amount)?;

                if !to_raw {
                    result = self.map_raw_to_percent(result)?;
                }

                Ok((result, to_raw))
            }
            Request::Get { to_raw } => {
                let mut result = self.backlight.brightness()?;

                if !to_raw {
                    result = self.map_raw_to_percent(result)?;
                }

                Ok((result, to_raw))
            }
            Request::GetMax => Ok((self.backlight.brightness_max()?, true)),
        }
    }

    fn map_percent_to_raw(&self, brightness_percent: i32) -> Result<i32, Box<dyn Error>> {
        Ok(Self::map_range(
            brightness_percent,
            0,
            100,
            0,
            self.backlight.brightness_max()?,
        ))
    }

    fn map_raw_to_percent(&self, brightness_raw: i32) -> Result<i32, Box<dyn Error>> {
        Ok(Self::map_range(
            brightness_raw,
            0,
            self.backlight.brightness_max()?,
            0,
            100,
        ))
    }

    /// Maps an i32 to be within the specified range.
    fn map_range(
        input: i32,
        input_start: i32,
        input_end: i32,
        output_start: i32,
        output_end: i32,
    ) -> i32 {
        output_end
            + (((output_end - output_start) * (input - input_end)) / (input_end - input_start))
    }
}
