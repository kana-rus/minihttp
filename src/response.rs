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
    pub(crate) fn into_byte_vec(&mut self) -> Vec<u8> {
        match self.status {
            Status::OK => {
                if let Some(json) = self.body.take() {
                    [
                        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_vec(),
                        json.0.to_vec()
                    ].concat()
                } else {
                    b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n".to_vec()
                }
            },
            Status::NotFound => b"HTTP/1.1 404 NotFound\r\n".to_vec(),
        }
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
    pub fn OK(body: JSON<'b>) -> Self {
        Self {
            status:  Status::OK,
            // headers: vec![],
            body: Some(body),
        }
    }
}