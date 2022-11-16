// use std::borrow::Cow;

use std::{net::TcpStream, io::Write};

use serde::{Serialize, Deserialize};
// use serde_bytes::Bytes;

use crate::{ServerResult, Response};

use super::consts::{BUF_SIZE, ENB};



// pub struct JSON<'b>(
//     pub Cow<'b, [u8]> //&'b [u8] //Vec<u8>
#[derive(Debug)]
pub struct JSON(
    [u8; BUF_SIZE] // Bytes
); impl<'de> JSON {
    pub fn from_struct<T: Serialize>(value: &T) -> ServerResult<Self> {
        let mut bytes = serde_json::to_vec(value)?;
        if bytes.len() > BUF_SIZE {
            return Err(Response::NotImplemented("I can't handle stream larger than 1024 bytes"))
        }
        bytes.resize(BUF_SIZE, ENB);
        let Ok(bytes) = TryInto::<[u8; BUF_SIZE]>::try_into(bytes) else {
            return Err(Response::InternalServerError("failed to hanlde json"))
        };
        Ok(Self(bytes))
    }

    pub fn to_struct<T: Deserialize<'de>>(&'de self) -> ServerResult<T> {
        // ======================================
        println!("try to desilialize: \"{}\"", std::str::from_utf8(self.body()).unwrap());
        // ======================================
        Ok(
            serde_json::from_slice(&self.body())?
        )
    }

    pub(crate) fn from_string_unchecked(string: String) -> Self {
        let mut bytes = string.as_bytes().to_vec();
        bytes.resize(BUF_SIZE, ENB);
        // let Ok(bytes) = TryInto::<[u8; BUF_SIZE]>::try_into(bytes) else {
        //     return Err(Response::InternalServerError())
        // };
        let bytes = TryInto::<[u8; BUF_SIZE]>::try_into(bytes).unwrap();
        Self(bytes)
    }
    pub(crate) fn body(&self) -> &[u8] {
        let end_of_body = 'eob: {
            for i in 0..BUF_SIZE {
                if self.0[i] == ENB {
                    break 'eob i
                }
            }
            BUF_SIZE
        };
        &self.0[..end_of_body]
    }
    pub(crate) fn write_body(self, stream: &mut TcpStream) -> std::io::Result<usize> {
        stream.write(&self.body())
    }

    // pub(crate) fn with_status_line(mut self, mut status_line: &[u8]) -> &[u8] {
    //     let end_of_body = &self.0.;
    //     for byte in self.0 {
    //         if byte == b'\\' {break}
    //         status_line.
    //     }
    // }
}
