use serde::Deserialize;
use crate::{JSON, ServerResult, Response};

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
} impl Method {
    pub(crate) fn parse(string: &str) -> ServerResult<Self> {
        match string {
            "GET"    => Ok(Self::GET),
            "POST"   => Ok(Self::POST),
            "PATCH"  => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            _ => Err(Response::BadRequest(format!("invalid request method: `{string}`")))
        }
    }
}

pub struct Request {
    // pub headers: Vec<HeaderOfReq>,
    pub(crate) body:    Option<JSON>,
}
impl<'d> Request {
    pub fn get_body<D: Deserialize<'d>>(&'d self) -> ServerResult<Option<D>> {
        let Some(json) = &self.body else {
            return Ok(None)
        };
        let body = json.to_struct()?;
        Ok(Some(body))
    }

    pub(crate) fn with_body_unchecked(body: &str) -> Self {
        Self { body: Some(JSON::from_string_unchecked(body.to_owned())) }
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