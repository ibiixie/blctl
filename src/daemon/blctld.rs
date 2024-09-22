use std::io::{Read, Write};

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
                    let mut request_data: Vec<u8> = Vec::new();
                    stream.read_to_end(&mut request_data).unwrap();

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
                    stream.write_all(response_data.as_slice()).unwrap();

                    println!("Communications successful!");
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
