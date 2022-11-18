use serde::{Serialize, Deserialize};

use crate::Context;


#[derive(Debug)]
pub struct JSON(pub(crate) String);
impl<'d> JSON {
    pub fn from_struct<S: Serialize>(value: &S) -> Context<Self> {
        Ok(Self(
            serde_json::to_string(value)?
        ))
    }
    pub fn to_struct<D: Deserialize<'d>>(&'d self) -> Context<D> {
        // =============================================
        // println!("[JSON::to_struct]: about to handle {:?}", self.0);
        // =============================================
        Ok(
            serde_json::from_str(&self.0)?
        )
    }

    pub(crate) fn from_string_unchecked(string: String) -> Self {
        Self(string)
    }
    // pub(crate) fn as_bytes(&self) -> &[u8] {
    //     self.0.as_bytes()
    // }
    pub(crate) fn content_length(&self) -> usize {
        self.0.len()
    }
}

