use serde::Deserialize;
use crate::{JSON, Context, Response};

#[derive(PartialEq, Eq, Hash, Debug)]
pub(crate) enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
} impl Method {
    pub(crate) fn parse(string: &str) -> Context<Self> {
        match string {
            "GET"    => Ok(Self::GET),
            "POST"   => Ok(Self::POST),
            "PATCH"  => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            _ => Response::BadRequest(format!("invalid request method: `{string}`"))
        }
    }
}

pub struct Request {
    // pub headers: Vec<HeaderOfReq>,
    pub(crate) body:    Option<JSON>,
}
impl<'d> Request {
    pub fn get_body<D: Deserialize<'d>>(&'d self) -> Context<Option<D>> {
        let Some(json) = &self.body else {
            return Ok(None)
        };
        let body = json.to_struct()?;
        Ok(Some(body))
    }
}