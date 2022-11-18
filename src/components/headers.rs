// pub enum HeaderOfReq {
//     
// }
// pub enum HeaderOfRes {
// 
// }

use chrono::{Utc, Local, DateTime};

use crate::{ServerResult, Response};


pub(crate) enum Header {
    ContentLength(usize),
    ContentType(ContentType),
    Date //(Date),
}
    pub(crate) enum ContentType {
        TextPlain,
        TextHTML,
        ApplicationJSON,
    }

impl Header {
    // pub(crate) fn parse(string: &'s str) -> ServerResult<Self> {
    //     match string.split_once(": ").ok_or_else(|| Response::BadRequest(
    //         format!("invalid request header: {string}"))
    //     )? {
    //         ("")
    // 
    //     }
    // }
    pub(crate) fn write_into_response(self, resp_buf: &mut String) {
        match self {
            Self::ContentLength(len) => {
                *resp_buf += "Content-Length: ";
                *resp_buf += &len.to_string();
            },
            Self::ContentType(content_type) => {
                *resp_buf += match content_type {
                    ContentType::TextPlain => "text/plain",
                    ContentType::TextHTML  => "text/html",
                    ContentType::ApplicationJSON => "application/json",
                };
            },
            Self::Date => {
                *resp_buf += "Date: ";
                *resp_buf += &Utc::now().to_rfc2822();
            },
        }
        *resp_buf += "\r\n"
    }
}
