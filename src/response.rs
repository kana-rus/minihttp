use std::{net::TcpStream, io::Write};

use chrono::Utc;

use crate::{
    JSON, Context,
};

#[derive(Debug)]
pub struct Response {
    status:  Status,
    // additinal_headers: [AdditionalHeader; 2],
    content_length: usize,
    body:    JSON,
}
#[derive(Debug)]
pub(crate) enum Status {
    OK                  = 200,
    BadRequest          = 400,
    NotFound            = 404,
    InternalServerError = 500,
    NotImplemented      = 501,
}

impl Response {
    pub(crate) fn write_to_stream(self, stream: &mut TcpStream) -> std::io::Result<usize> {
        match &self.status {
            other => {
                stream.write(
                    match other {
                        Status::OK                  => b"HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nConnection: Keep-Alive\r\nKeep-Alive: timeout=5\r\n",
                        Status::BadRequest          => b"HTTP/1.1 400 BadRequest\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: Keep-Alive\r\nKeep-Alive: timeout=5\r\n",
                        Status::InternalServerError => b"HTTP/1.1 500 InternalServerError\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: Keep-Alive\r\nKeep-Alive: timeout=5\r\n",
                        Status::NotFound            => b"HTTP/1.1 404 NotFound\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: Keep-Alive\r\nKeep-Alive: timeout=5\r\n",
                        Status::NotImplemented      => b"HTTP/1.1 501 NotImplemented\r\nContent-Type: text/plain; charset=utf-8\r\nConnection: Keep-Alive\r\nKeep-Alive: timeout=5\r\n",
                    }
                )?;
                stream.write(format!(
                    "Content-Length: {}\r\nDate: {}\r\n\r\n{}",
                    self.content_length,
                    Utc::now().to_rfc2822(),
                    
                    self.body.0
                ).as_bytes())
            },
        }
    }

    #[allow(non_snake_case)]
    pub fn OK(body: JSON) -> Context<Self> {
        Ok(Self {
            status:  Status::OK,
            content_length: body.content_length(),
            body: body,
        })
    }
    #[allow(non_snake_case)]
    pub fn NotFound<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status: Status::NotFound,
            content_length: msg.len(),
            body: JSON::from_string_unchecked(msg),
        })
    }
    #[allow(non_snake_case)]
    pub fn BadRequest<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status:  Status::BadRequest,
            content_length: msg.len(),
            body: JSON::from_string_unchecked(msg),
        })
    }
    #[allow(non_snake_case)]
    pub fn InternalServerError<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status:  Status::InternalServerError,
            content_length: msg.len(),
            body: JSON::from_string_unchecked(msg),
        })
    }
    #[allow(non_snake_case)]
    pub fn NotImplemented<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status:  Status::NotImplemented,
            content_length: msg.len(),
            body: JSON::from_string_unchecked(msg),
        })
    }
}