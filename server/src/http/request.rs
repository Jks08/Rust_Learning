use super::method::Method;
use std::convert::TryFrom;

pub struct Request{
    // Rust does not support NULL values, but uses enum from Option, from 
    // standard library. It is a pipe safe way to not encounter no pointer 
    // exceptions.
    path: String,
    query_string: Option<String>,
    method: Method,
}

// Traits are similar to interfaces in other languages.
// Trait conversion using From trait. 
impl TryFrom<&[u8]> for Request{
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}