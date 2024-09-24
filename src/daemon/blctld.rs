use std::io::prelude::*;

use crate::backlight::{Backlight, Sysfs};

use blctl_shared::{Request, Response};

pub struct Daemon {
    // backlight: Box<dyn Backlight>
}

impl Daemon {
    pub fn new() -> Self {
        let listener = std::os::unix::net::UnixListener::bind("/tmp/blctld.sock")
            .unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut message_length = [0u8; std::mem::size_of::<usize>()];
                    stream.read_exact(&mut message_length).unwrap();
            
                    let message_length = usize::from_ne_bytes(message_length);
                    println!("Message received, length is {message_length}");

                    // let mut request_data: Vec<u8> = Vec::new();
                    // stream.read_to_end(&mut request_data).unwrap();
                    let mut request_data: Vec<u8> = vec![0u8; message_length];
                    stream.read_exact(&mut request_data).unwrap();
                    println!("Socket read success");

                    let request = bincode::deserialize::<Request>(request_data.as_slice()).unwrap();
                    dbg!(request);

                    let response = match request {
                        Request::Set { level, raw } => Response::Success { level, raw },
                        Request::Increase { amount, raw } => Response::Success { level: amount, raw },
                        Request::Decrease { amount, raw } => Response::Success { level: amount, raw },
                        Request::Get { raw } => Response::Success { level: 255, raw },
                        Request::GetMax => Response::Success { level: 255, raw: true },
                        Request::Store => Response::Success { level: 255, raw: true },
                        Request::Restore => Response::Success { level: 255, raw: true },
                    };

                    let response_data = bincode::serialize(&response).unwrap();
                    println!("Response data size is {}", response_data.len());

                    stream.write_all(&response_data.len().to_ne_bytes()).unwrap();
                    stream.write_all(&response_data).unwrap();
                    stream.flush().unwrap();

                    println!("Socket write success");
                },
                Err(err) => {
                    break;
                }
            }
        }

        Self {
            // backlight: Box::new(Sysfs::new());
        }
    }
}
