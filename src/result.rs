// use std::ops::{Try, FromResidual, ControlFlow};
use crate::Response;


pub type ServerResult<T> = std::result::Result<T, Response>;
// pub enum ServerResult<T> {
//     Ok(T),
//     Err(Response),
// }

impl From<std::io::Error> for Response {
    fn from(value: std::io::Error) -> Self {
        Self::InternalServerError()
    }
}
impl From<serde_json::Error> for Response {
    fn from(value: serde_json::Error) -> Self {
        Self::InternalServerError()
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