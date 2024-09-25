use std::io::prelude::*;
use std::path::Path;
use std::os::unix::net::UnixStream;

use blctl_shared::{Request, Response};

pub struct IpcClient {
    socket: UnixStream
}

impl IpcClient {
    pub fn new(path: &Path) -> Self {
        let socket = UnixStream::connect(path)
            .expect("unable to connect to daemon socket (is blctld running?)");
        
        Self {
            socket
        }
    }

    pub fn request(&mut self, request: Request) -> Response {
        println!("Making request to daemon");

        let request_data = bincode::serialize(&request).unwrap();
        println!("Request data size is {} bytes", request_data.len());

        self.socket.write_all(&request_data.len().to_ne_bytes())
            .expect("error while writing request header to daemon socket");
        self.socket.write_all(request_data.as_slice())
            .expect("error while writing request data to daemon socket");

        println!("Request sent to daemon");
        
        println!("Awaiting response from daemon");

        let mut response_size = [0u8; std::mem::size_of::<usize>()];
        self.socket.read_exact(&mut response_size)
            .expect("error while reading response header from daemon socket");

        let response_size = usize::from_ne_bytes(response_size);

        println!("Response data size is {response_size} bytes");

        let mut response_data = vec![0u8; response_size];
        self.socket.read_exact(&mut response_data)
            .expect("error while reading response data from daemon socket");

        let response = bincode::deserialize::<Response>(&response_data).unwrap();

        println!("Response received from daemon");
        dbg!(&response);

        response
    }
}