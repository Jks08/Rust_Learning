
pub enum StatusCode {
    Ok200 = 200,
    NotFound404 = 404,
}

pub struct Response{
    status_code: StatusCode,
    body: Option<String>,
}

impl Response{
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self{
        Response{
            status_code,
            body,
        }
    }
}