// use std::ops::{Try, FromResidual, ControlFlow};
use crate::Response;

pub type Context<T> = std::result::Result<T, Response>;
// pub enum ServerResult<T> {
//     Ok(T),
//     Err(Response),
// }

impl From<std::io::Error> for Response {
    fn from(value: std::io::Error) -> Self {
        Self::InternalServerError::<String, ()>(value.to_string() + ": caused by I/O").unwrap_err()
    }
}
impl From<serde_json::Error> for Response {
    fn from(value: serde_json::Error) -> Self {
        Self::InternalServerError::<String, ()>(value.to_string() + ": caused by json handling :: " + {
            if value.is_data() {
                "invalid json data"
            } else if value.is_eof() {
                "unexpected end of line"
            } else if value.is_io() {
                "about io"
            } else {  // value.is_syntax()
                "wrong json syntax"
            }
        }).unwrap_err()
    }
} 
impl From<std::str::Utf8Error> for Response {
    fn from(value: std::str::Utf8Error) -> Self {
        Self::InternalServerError::<String, ()>(value.to_string() + ": caused by UTF-8 handling").unwrap_err()
    }
}

// impl FromResidual<Response> for ServerResult<Response> {
//     fn from_residual(residual: Response) -> Self {
//         match residual.status {
//             OK => Self::Ok(residual),
//             _ => Self::Err(residual),
//         }
//     }
// }
// impl Try for ServerResult<Response> {
//     type Residual = Response;
//     type Output = Response;
//     fn from_output(output: Self::Output) -> Self {
//         Self::Ok(output)
//     }
//     fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
//         match self {
//             ServerResult::Ok(response) => ControlFlow::Continue(response),
//             Self::Err(response) => ControlFlow::Break(response),
//         }
//     }
// }