#![allow(dead_code)]

use server::Server;
use http::Method;
use http::Request;

mod server; // To import from other file: server.rs
mod http;

// Rust does not handle exceptions. It uses Result enum.
// Result enum handles recoverable errors.

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}