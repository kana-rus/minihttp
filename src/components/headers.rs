use std::{net::TcpStream, io::Write};

use chrono::Utc;


#[derive(Debug)]
pub(crate) enum AdditionalHeader {
    ContentLength(usize),
    Date //(Date),
}

impl AdditionalHeader {
    pub(crate) fn write_to_stream(self, stream: &mut TcpStream) -> std::io::Result<usize> {
        match self {
            Self::ContentLength(len) => {
                stream.write(b"Content-Length: ")?;
                stream.write(len.to_string().as_bytes())?;
            },
            Self::Date => {
                stream.write(b"Date: ")?;
                stream.write(Utc::now().to_rfc2822().as_bytes())?;
            },
        }
        stream.write(b"\r\n")
    }
}
