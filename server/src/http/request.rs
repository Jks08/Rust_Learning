use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;

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
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(buf){
        //     Ok(request) => {},
        //     Err(_) => return Err(ParseError::InvalidEncoding), 
        // }

        // match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)){
        //     Ok(request) => {},
        //     Err(e) => return Err(e),
        // }

        let request = str::from_utf8(buf)?;
        // match get_next_word(request){
        //     Some((method, request)) => {},
        //     None => return Err(ParseError::InvalidRequest),

        // }

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }
        // ? will try to convert the result to type.
        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str,&str)>{
    // let mut iter = request.chars();
    // loop {
    //     let item = iter.next();
    //     match item{
    //         Some(c) => {},
    //         None => break,
    //     }
    // }

    for (i,c) in request.chars().enumerate(){
        if c == ' ' || c == '\r'{
            return Some((&request[..i], &request[i+1..]));
        }
    }
    None
}

pub enum ParseError{
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError{
    fn message(&self) -> &str{
        match self{
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Parse Error {}", self.message())
    }
}

impl Debug for ParseError{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "Parse Error {}", self.message())
    }
}

impl Error for ParseError{}