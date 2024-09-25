use std::{io::prelude::*, os::unix::net::{UnixListener, UnixStream}, path::Path};

use crate::backlight::{Backlight, Sysfs};

use blctl_shared::{Request, Response};

pub struct Daemon {
    listener: UnixListener
    // backlight: Box<dyn Backlight>
}

impl Daemon {
    pub fn new(path: &Path) -> Self {
        if path.exists() {
            // Socket file exists, try to delete so we can bind
            println!("Warning: socket already exists - trying to delete");
            std::fs::remove_file(path)
                .expect("unable to remove daemon socket (is it still in use?)");
        }

        let listener = UnixListener::bind(path)
            .expect("unable to bind to daemon socket");

        println!("Bound to daemon socket");

        Self {
            listener
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

        let response = match request {
            Request::Set { level, raw } => Response::Success { level, raw },
            Request::Increase { amount, raw } => Response::Success { level: amount, raw },
            Request::Decrease { amount, raw } => Response::Success { level: amount, raw },
            Request::Get { raw } => Response::Success { level: 100, raw },
            Request::GetMax => Response::Success { level: 100, raw: true },
            Request::Store => Response::Success { level: 100, raw: false },
            Request::Restore => Response::Success { level: 100, raw: false },
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