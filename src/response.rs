use std::{net::TcpStream, io::Write};

use crate::{
    // components::headers::HeaderOfRes,
    JSON,
};

#[derive(Debug)]
pub struct Response {
    pub(crate) status:  Status,
    // headers: Vec<HeaderOfRes>,
    body:      Option<JSON>,
}
#[derive(Debug)]
pub(crate) enum Status {
    OK                  = 200,
    BadRequest          = 400,
    NotFound            = 404,
    InternalServerError = 500,
    NotImplemented      = 501,
}

// trait ResponseBody {}
// impl ResponseBody for JSON {}
// impl ResponseBody for Text {}


impl Response {
    pub(crate) fn write_to_stream(mut self, stream: &mut TcpStream) -> std::io::Result<usize> {
        match self.status {
            Status::OK => {
                if let Some(json) = self.body.take() {
                    stream.write(b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n")?;
                    stream.write(json.as_bytes())
                } else {
                    stream.write(b"HTTP/1.1 200 OK\r\n")
                }
            },

            Status::BadRequest => {
                stream.write(b"HTTP/1.1 400 BadRequest\r\nContent-Type: application/json\r\n\r\n")?;
                stream.write(self.body.unwrap().as_bytes())
            },
            Status::InternalServerError => {
                stream.write(b"HTTP/1.1 500 InternalServerError\r\nContent-Type: application/json\r\n\r\n")?;
                stream.write(self.body.unwrap().as_bytes())
            },
            Status::NotImplemented => {
                stream.write(b"HTTP/1.1 501 NotImplemented\r\nContent-Type: application/json\r\n\r\n")?;
                stream.write(self.body.unwrap().as_bytes())
            },

            Status::NotFound => stream.write(b"HTTP/1.1 404 NotFound\r\n"),
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
    #[allow(non_snake_case)]
    pub fn NotFound() -> Self {
        Self {
            status:  Status::NotFound,
            // headers: vec![],
            body:    None,
        }
    }
    #[allow(non_snake_case)]
    pub fn BadRequest<Msg: ToString>(msg: Msg) -> Self {
        Self {
            status:  Status::BadRequest,
            // headers: vec![],
            body:    Some(JSON::from_string_unchecked(msg.to_string())),
        }
    }
    #[allow(non_snake_case)]
    pub fn InternalServerError<Msg: ToString>(msg: Msg) -> Self {
        Self {
            status:  Status::InternalServerError,
            // headers: vec![],
            body:    Some(JSON::from_string_unchecked(msg.to_string())),
        }
    }
    #[allow(non_snake_case)]
    pub fn NotImplemented<Msg: ToString>(msg: Msg) -> Self {
        Self {
            status:  Status::NotImplemented,
            // headers: vec![],
            body:    Some(JSON::from_string_unchecked(msg.to_string())),
        }
    }
}