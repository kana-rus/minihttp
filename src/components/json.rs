// use std::borrow::Cow;

use std::{net::TcpStream, io::Write};

use serde::{Serialize, Deserialize};
// use serde_bytes::Bytes;

use crate::{ServerResult, Response};

use super::consts::BUF_SIZE;

const ENB: u8 = b'^';

// pub struct JSON<'b>(
//     pub Cow<'b, [u8]> //&'b [u8] //Vec<u8>
#[derive(Debug)]
pub struct JSON(
    pub(crate) [u8; BUF_SIZE] // Bytes
); impl<'de> JSON {
    pub fn from_struct<T: Serialize>(value: &T) -> ServerResult<Self> {
        let mut bytes = serde_json::to_vec(value)?;
        if bytes.len() > BUF_SIZE {
            return Err(Response::NotImplemented())
        }
        bytes.resize(BUF_SIZE, ENB);
        let Ok(bytes) = TryInto::<[u8; BUF_SIZE]>::try_into(bytes) else {
            return Err(Response::InternalServerError())
        };
        Ok(Self(bytes))
    }

    pub fn to_struct<T: Deserialize<'de>>(&'de self) -> ServerResult<T> {
        Ok(
            serde_json::from_slice(&self.0)?
        )
    }

    pub(crate) fn write_body(self, stream: &mut TcpStream) -> std::io::Result<usize> {
        let end_of_body = 'eob: {
            for i in 0..BUF_SIZE {
                if self.0[i] == ENB {
                    break 'eob i
                }
            }
            BUF_SIZE
        };
        stream.write(&self.0[..end_of_body])
    }

    // pub(crate) fn with_status_line(mut self, mut status_line: &[u8]) -> &[u8] {
    //     let end_of_body = &self.0.;
    //     for byte in self.0 {
    //         if byte == b'\\' {break}
    //         status_line.
    //     }
    // }
}
