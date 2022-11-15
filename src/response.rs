use crate::{components::headers::HeaderOfRes, JSON};

pub struct Response<'b> {
    status:  Status,
    // headers: Vec<HeaderOfRes>,
    body:    Option<JSON<'b>>,
}

enum Status {
    OK       = 200,
    NotFound = 404,
}

impl<'b> Response<'b> {
    pub(crate) fn into_bytes(&self) -> &[u8] {
        let mut serialized = match self.status {
            Status::OK       => "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"ok\": true}",
            Status::NotFound => "HTTP/1.1 404 NotFound\r\n",
        };
        // for header in self.headers {
        // 
        // }
        // self.body
        serialized.as_bytes()
    }
    #[allow(non_snake_case)]
    pub fn NotFound() -> Self {
        Self {
            status:  Status::NotFound,
            // headers: vec![],
            body:    None,
        }
    }
    #[allow(non_snake_case)]
    pub fn OK(body: JSON) -> Self {
        Self {
            status:  Status::OK,
            // headers: vec![],
            body: Some(body),
        }
    }
}