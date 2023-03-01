use server::Server;
// use http::request::Request;
// use http::method::Method;
use http::Method;
use http::Request;

mod server; // To import from other file: server.rs
mod http;

fn main() {
    // let get = Method::GET;
    // let delete = Method::DELETE;
    // let post = Method::POST;
    // let put = Method::PUT;
    
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
    // let string = String::from("127.0.0.1:8080");
    // let string_slice = &string[10..]; // this means everything after 10th byte
    // let string_borrow: &str = &string;
    // let string_literal = "1234";

    // //from 10th index till last. [10..14] will give in that range.
    // dbg!(&string);
    // dbg!(string_slice);
    // dbg!(string_borrow);
    // dbg!(string_literal);
    // dbg!(&string2);
}

// mod server{
//     // Struct block
//     // If we don't put pub keywrod, then it is private by default.

//     pub struct Server{
//         addr: String,
//     }

//     // Implementation block
//     impl Server{
//         pub fn new(addr: String)->Self{
//             Self{
//                 addr: addr
//             }
//         }
//         // run takes ownership of entire struct. 
//         // If we pass in &mut self, it won't take ownership.
//        pub fn run(self){
//             println!("Listening on {}", self.addr)
//         }
//     }
// }

// mod http{
//     pub mod request{
//         // use super::method::Method;
//         // pub struct Request{
//         //     // Rust does not support NULL values, but uses enum from Option, from 
//         //     // standard library. It is a pipe safe way to not encounter no pointer 
//         //     // exceptions.
//         //     path: String,
//         //     query_string: Option<String>,
//         //     method: Method,
//         // }
//     }

//     // pub mod method{
//     //     pub enum Method{
//     //         GET,
//     //         DELETE,
//     //         POST,
//     //         PUT,
//     //         HEAD,
//     //         CONNECT,
//     //         OPTIONS,
//     //         TRACE,
//     //         PATCH,
//     //     }
//     // }
// }