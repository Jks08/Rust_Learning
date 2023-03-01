    // Struct block
    // If we don't put pub keywrod, then it is private by default.

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
    pub fn run(self){
        println!("Listening on {}", self.addr)
    }
}