use std::{net::TcpStream, io::Write};

use crate::{
    JSON, components::headers::AdditionalHeader, Context,
};

#[derive(Debug)]
pub struct Response {
    status:  Status,
    additinal_headers: [AdditionalHeader; 2],
    body:    Option<JSON>,
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
                for header in self.additinal_headers {
                    header.write_to_stream(stream)?;
                }
                stream.write(b"\r\n")?;
                stream.write(self.body.unwrap().as_bytes())
            },
        }
    }

    #[allow(non_snake_case)]
    pub fn OK(body: JSON) -> Context<Self> {
        Ok(Self {
            status:  Status::OK,
            additinal_headers: [
                AdditionalHeader::ContentLength(body.content_length()),
                AdditionalHeader::Date,
            ],
            body: Some(body),
        })
    }
    #[allow(non_snake_case)]
    pub fn NotFound<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status: Status::NotFound,
            additinal_headers: [
                AdditionalHeader::ContentLength(msg.len()),
                AdditionalHeader::Date,
            ],
            body: Some(JSON::from_string_unchecked(msg)),
        })
    }
    #[allow(non_snake_case)]
    pub fn BadRequest<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status:  Status::BadRequest,
            additinal_headers: [
                AdditionalHeader::ContentLength(msg.len()),
                AdditionalHeader::Date,
            ],
            body: Some(JSON::from_string_unchecked(msg)),
        })
    }
    #[allow(non_snake_case)]
    pub fn InternalServerError<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status:  Status::InternalServerError,
            additinal_headers: [
                AdditionalHeader::ContentLength(msg.len()),
                AdditionalHeader::Date,
            ],
            body: Some(JSON::from_string_unchecked(msg)),
        })
    }
    #[allow(non_snake_case)]
    pub fn NotImplemented<Msg: ToString, T>(msg: Msg) -> Context<T> {
        let msg = msg.to_string();
        Err(Self {
            status:  Status::NotImplemented,
            additinal_headers: [
                AdditionalHeader::ContentLength(msg.len()),
                AdditionalHeader::Date,
            ],
            body: Some(JSON::from_string_unchecked(msg)),
        })
    }
}