use super::method::{MethodError,Method};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::str;
use std::str::Utf8Error;
use super::QueryString;

// Rust does not support NULL values, but uses enum from Option, from 
// standard library. It is a pipe safe way to not encounter no pointer 
// exceptions.
#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_string: Option<QueryString<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }
}

// Traits are similar to interfaces in other languages.
// Trait conversion using From trait. 
impl<'buf> TryFrom<&'buf [u8]> for Request<'buf>{
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
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
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1"{
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let mut query_string = None;
        // match path.find('?'){
        //     Some(i) => {
        //         query_string = Some(path[i+1..].to_string());
        //         path = &path[..i];
        //     }
        //     None => {}
        // }
        // let q = path.find('?');
        // if q.is_some(){
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]);
        //     path = &path[..i];
        // }

        if let Some(i) = path.find('?'){
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }
        // ? will try to convert the result to type.
        Ok(Self{
            path: path,
            query_string,
            method,
        })
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

impl From<MethodError> for ParseError{
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
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