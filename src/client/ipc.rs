use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::path::Path;

use blctl_shared::{Request, Response};

pub struct Client {
    socket: UnixStream,
}

impl Client {
    pub fn new(path: &Path) -> Self {
        let socket = UnixStream::connect(path)
            .expect("unable to connect to daemon socket (is blctld running?)");

        Self { socket }
    }

    pub fn request(&mut self, request: Request) -> Response {
        let request_data = bincode::serialize(&request).unwrap();

        self.socket
            .write_all(&request_data.len().to_ne_bytes())
            .expect("error while writing request header to daemon socket");

        self.socket
            .write_all(request_data.as_slice())
            .expect("error while writing request data to daemon socket");

        let mut response_size = [0u8; std::mem::size_of::<usize>()];
        self.socket
            .read_exact(&mut response_size)
            .expect("error while reading response header from daemon socket");

        let response_size = usize::from_ne_bytes(response_size);

        let mut response_data = vec![0u8; response_size];
        self.socket
            .read_exact(&mut response_data)
            .expect("error while reading response data from daemon socket");

        let response = bincode::deserialize::<Response>(&response_data).unwrap();

        dbg!(&response);

        response
    }
}
