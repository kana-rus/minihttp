use std::borrow::Cow;

use serde::{Serialize, Deserialize};
use super::result::ServerError;


pub struct JSON<'b>(
    pub Cow<'b, [u8]> //&'b [u8]// Vec<u8>
); impl<'b> JSON<'b> {
    pub fn from_struct<T: Serialize>(value: T) -> Self{//Result<Self, ServerError> {
        // Ok(
            Self(
                Cow::Owned(
                    serde_json::to_vec(&value).unwrap()//?
                )
            )
        // )
    }
    pub fn to_struct<T: Deserialize<'b>>(&'b self) -> T {// Result<T, ServerError> {
        // Ok(
            serde_json::from_slice(&self.0).unwrap()//?
        // )
    }
    // pub fn into_struct<T: Deserialize<'b>>(self) -> Result<T, ServerError> {
    //     Ok(
    //         serde_json::from_slice(self.0)?
    //     )
    // }
}


