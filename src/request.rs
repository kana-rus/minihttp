use serde::Deserialize;

use crate::{JSON, components::result::ServerError};

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Method {
    GET,
    POST,
}

pub struct Request<'b> {
    // pub headers: Vec<HeaderOfReq>,
    pub body:    Option<JSON<'b>>,
}

impl<'b> Request<'b> {
    // pub fn get_body<T: Deserialize<'b>>(&'b mut self) -> Result<T, ServerError> {
    //     // let Some(body) = self.body else {
    //     //     return Err(ServerError::JSONerror("body is None".into()))
    //     // };
    //     // Ok(
    //     //     serde_json::from_slice(body.0)?
    //     // )
    //     let body = self.body.take();
    //     if body.is_none() {
    //         return Err(ServerError::JSONerror("body is None".into()))
    //     }
    //     let body = body.unwrap();
    //         Ok(serde_json::from_slice(&body.0)?)
    // }
}