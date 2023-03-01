// Struct block
// If we don't put pub keywrod, then it is private by default.
use std::net::TcpListener;

pub struct Server{
    addr: String,
}

// Implementation block
impl Server{
    pub fn new(addr: String)->Self{
        Self{
            addr: addr
        }
    }
    // run takes ownership of entire struct. 
    // If we pass in &mut self, it won't take ownership.
    pub fn run(self) -> (i32, &str, std::net::TcpListener){
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap(); // recoverable error 

        // while true{} // Normal infinite loop, but Rust has the following special:
        loop{
            listener.accept();
        }
        // let tup = (5,"a",listener);
        (5,"a",listener)
    }
}