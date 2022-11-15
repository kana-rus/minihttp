use crate::JSON;

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
    pub fn body<T>(&self) -> T {

    }
}