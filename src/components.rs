pub type ServerResult = Result<(), ServerError>;
#[derive(Debug)]
pub enum ServerError {
    IOError(std::io::Error)
}
impl From<std::io::Error> for ServerError {
    fn from(value: std::io::Error) -> Self {
        Self::IOError(value)
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Method {
    GET,
    POST,
}

pub struct JSON();
pub struct Request {
    headers: Vec<HeaderOfReq>,
    body:    Option<JSON>,
}
pub struct Response {
    status:  Status,
    headers: Vec<HeaderOfRes>,
    body:    JSON,
}
enum HeaderOfReq {
    
}
enum HeaderOfRes {

}
enum Status {
    OK       = 200,
    NotFound = 404,
}

impl Response {
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
            headers: vec![],
            body:    JSON()
        }
    }
    #[allow(non_snake_case)]
    pub fn OK(body: JSON) -> Self {
        Self {
            status:  Status::OK,
            headers: vec![],
            body,
        }
    }
}

pub(crate) const BUF_SIZE: usize = 1024;
pub(crate) fn parse_stream(buffer: &[u8; BUF_SIZE]) -> Result<(&str, Method, Request), ServerError> {
    let request_status = {
        let mut end_of_reqest_status = BUF_SIZE;
        for pos in 0..BUF_SIZE {
            if buffer[pos]   == b'\r'  
            && buffer[pos+1] == b'\n' {
                if pos == 0 {
                    return Err(ServerError::IOError(
                        std::io::Error::new(
                            std::io::ErrorKind::Other,
                            "HTTP request starts with '\\r'"
                        )
                    ))
                }
                end_of_reqest_status = pos - 1;
                break
            }
        }
        if end_of_reqest_status == BUF_SIZE {
            return Err(ServerError::IOError(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "HTTP request doesn't contain any valid request status",
                )
            ))
        }
        &buffer[..=end_of_reqest_status]
    };

    let mut split = request_status.split(|b| *b == b' ');
    let method = match split.next().expect("no method found in request") {
        b"GET"  => Method::GET,
        b"POST" => Method::POST,
        _ => return Err(ServerError::IOError(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "HTTP request doesn't contain any valid method",
            )
        ))
    };
    let path = std::str::from_utf8(
        split.next().expect("no request path found in request")
    ).expect("failed to get path from buffer");

    Ok((
        path,
        method,
        Request {
            headers: vec![],
            body:    None,
        }
    ))
}