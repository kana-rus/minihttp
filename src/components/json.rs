// use std::borrow::Cow;

use std::{net::TcpStream, io::Write};

use serde::{Serialize, Deserialize};
// use serde_bytes::Bytes;

use crate::{ServerResult, Response};

use super::consts::{
    BUF_SIZE,
    // ENB
};


/*
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
    }
*/
#[derive(Debug)]
pub struct JSON(String);
impl<'d> JSON {
    pub fn from_struct<S: Serialize>(value: &S) -> ServerResult<Self> {
        Ok(Self(
            serde_json::to_string(value)?
        ))
    }
    pub fn to_struct<D: Deserialize<'d>>(&'d self) -> ServerResult<D> {
        // =============================================
        println!("[JSON::to_struct]: about to handle {:?}", self.0);
        // =============================================
        Ok(
            serde_json::from_str(&self.0)?
        )
    }
    // pub(crate) fn from_raw_request_body(body: &str) -> Self {
    //     // let quote: &[_] = &['\'', '"'];
    //     // Self(body.trim_matches(quote).to_owned())
    // 
    //     // =============================================
    //     println!("[JSON::from_raw_request_body]: about to handle {:?}", body);
    //     // =============================================
    //     Self(body.trim().to_owned())
    // }
    pub(crate) fn from_string_unchecked(string: String) -> Self {
        Self(string)
    }
    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

