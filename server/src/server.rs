// Struct block
// If we don't put pub keywrod, then it is private by default.
use std::net::TcpListener;
use crate::http::request::Request;
use std::io::Read;
use std::convert::TryFrom;

pub struct Server{
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
    pub fn run(self){
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
                            // If we use {:?}, it will be debug format, not display.

                            match Request::try_from(&buffer[..]);{
                                Ok(request) => {},
                                Err(e) => println!("Failed to parse a request: {}", e),
                            }
                            // let res: &Result<Request, _> = &buffer[..].try_into();
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                    // sample array : 
                    // let a = [1,2,3,4,5]; // type is [i32;5]
                    // arr(&a);
                }
                Err(e) => {
                    println!("Connection failed: {}", e);
                }
            }
            // let res = listener.accept();
            // if res.is_err(){
            //     continue;
            // }

            // let (stream,addr) = res.unwrap();
        } // Not the best way to handle enum errors. Match is better.

        // let tup = (5,"a",listener);
        // (5,"a",listener)
    }
}