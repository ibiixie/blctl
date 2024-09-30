use std::fs::Permissions;
use std::io::{Read, Write};
use std::path::Path;

use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};

use crate::backlight::{Backlight, Sysfs};

use blctl_shared::{Request, Response};

pub struct Daemon {
    listener: UnixListener,
    backlight: Box<dyn Backlight>
}

impl Daemon {
    pub fn new(path: &Path) -> Self {
        let backlight = Box::new(Sysfs::new()
            .expect("unable to create sysfs backlight interface"));

        if path.exists() {
            println!("Removing old socket");

            std::fs::remove_file(path)
                .expect("unable to remove unused socket");
        } else {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        }

        let listener = UnixListener::bind(path)
            .expect("unable to bind to daemon socket");

        println!("Bound to daemon socket");

        std::fs::set_permissions(path, Permissions::from_mode(0o666))
            .expect("failed to set socket permissions");

        println!("Set socket file permissions to 666");

        Self {
            listener,
            backlight
        }
    }

    pub fn listen(&self) {
        println!("Awaiting connections");

        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("Connection accepted");
                    self.handle_client(&stream);
                },
                Err(err) => {
                    println!("Unable to accept client connection - skipping: {err}");
                    continue;
                }
            }
        }
    }

    fn handle_client(&self, client_stream: &UnixStream) {
        println!("Handling client");

        let request = self.process_request(client_stream);
        self.process_response(client_stream, request);
    }

    fn process_request(&self, mut client_stream: &UnixStream) -> Request {
        println!("Processing request");

        let mut request_size = [0u8; std::mem::size_of::<usize>()];
        client_stream.read_exact(&mut request_size)
            .expect("error while reading response header from client connection");

        let request_size = usize::from_ne_bytes(request_size);

        println!("Request data size is {request_size} bytes");

        let mut request_data = vec![0u8; request_size];
        client_stream.read_exact(&mut request_data)
            .expect("error while reading request data from client connection");

        let request = bincode::deserialize::<Request>(&request_data).unwrap();

        println!("Request received from client");
        dbg!(&request);

        request
    }

    fn process_response(&self, mut client_stream: &UnixStream, request: Request) {
        println!("Processing response");

        // Todo: Handle backlight brightness value mapping

        let response = match request {
            Request::Set { level, raw } => {
                self.backlight.set_brightness(level)
            },
            Request::Increase { amount, raw } => {
                match self.backlight.brightness() {
                    Ok(brightness) => {
                        self.backlight.set_brightness(brightness + amount)
                    }
                    Err(err) => Err(err)
                }
            },
            Request::Decrease { amount, raw } => {
                match self.backlight.brightness() {
                    Ok(brightness) => {
                        self.backlight.set_brightness(brightness - amount)
                    }
                    Err(err) => Err(err)
                }
            },
            Request::Get { raw } => {
                self.backlight.brightness()
            },
            Request::GetMax => {
                self.backlight.brightness_max()
            },
            Request::Store => {
                todo!()
            },
            Request::Restore => {
                todo!()
            },
        };

        let response = match response {
            Ok(level) => Response::Success { level, raw: true },
            Err(err) => Response::Failure { reason: err.to_string() }
        };

        let response_data = bincode::serialize(&response).unwrap();
        println!("Response data size is {} bytes", response_data.len());

        client_stream.write_all(&response_data.len().to_ne_bytes())
            .expect("error while writing response header to client connection");
        client_stream.write_all(&response_data)
            .expect("error while writing response data to client connection");

        println!("Response sent to client");
    }
}