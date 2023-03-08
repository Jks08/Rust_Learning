use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Copy, Clone, Debug)]
// Copy and Clone are used to make the enum copyable and clonable.
// Copy lives on the stack, Clone lives on the heap.
pub enum StatusCode{
    Ok = 200,
    NotFound = 404,
    BadRequest = 400,
}

impl StatusCode{
    pub fn reason_phrase(&self) -> &str{
        match self{
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
            Self::BadRequest => "Bad Request",
        }
    }
}

impl Display for StatusCode{
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        write!(f, "{}", *self as u16)
    }
}