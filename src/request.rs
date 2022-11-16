use serde::Deserialize;
use crate::{JSON, ServerResult};

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Method {
    GET,
    POST,
}

pub struct Request {
    // pub headers: Vec<HeaderOfReq>,
    pub body:    Option<JSON>,
}
impl<'d> Request {
    pub fn get_body<D: Deserialize<'d>>(&'d self) -> ServerResult<Option<D>> {
        let Some(json) = &self.body else {
            return Ok(None)
        };
        let body = json.to_struct()?;
        Ok(Some(body))
    }
}

// impl Request {
//     pub fn get_body<T: Deserialize>(&'b mut self) -> ServerResult<Option<T>> {
//         let Some(body) = self.body.take() else {
//             return Ok(None);
//         };
//         let body = serde_json::from_slice::<T>(&body.0)?;
//         Ok(Some(body))
//     }
// }