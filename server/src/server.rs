// Struct block
// If we don't put pub keywrod, then it is private by default.
use crate::http::{ParseError, Request, Response, StatusCode};
use std::net::TcpListener;
use std::io::{Write,Read};
use std::convert::TryFrom;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

// fn arr(a: &[u8]){}

// Implementation block
impl Server{
    pub fn new(addr: String)->Self{
        Self{
            addr: addr
        }
    }
    // run takes ownership of entire struct. 
    // If we pass in &mut self, it won't take ownership.
    pub fn run(self, mut handler: impl Handler){
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap(); 
        // recoverable error 

        // while true{} // Normal infinite loop, but Rust has the following special:
        loop{
            match listener.accept(){ // Need to specify Ok and Err
                // Ok((stream, _)) => { don't care about the second value }
                // Ok((stream, addr)) => { addr is a tuple } 
                Ok((mut stream, _)) => {
                    let mut buffer = [0;1024]; // 0 is initial value, 1024 is size
                    match stream.read(&mut buffer){
                        Ok(_) => {
                            println!("Received Request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}