use super::method::Method;
pub struct Request{
    // Rust does not support NULL values, but uses enum from Option, from 
    // standard library. It is a pipe safe way to not encounter no pointer 
    // exceptions.
    path: String,
    query_string: Option<String>,
    method: Method,
}