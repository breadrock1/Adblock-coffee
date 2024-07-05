use jni::errors::{Exception, ToException};

use adblock::request::RequestError;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustException {
    #[error("Failed while creating request: {0}")]
    CreateRequest(String),
    #[error("Failed while extracting parameter: {0}")]
    ExtractParameter(String),
}

impl ToException for RustException {
    fn to_exception(&self) -> Exception {
        Exception {
            class: "RustException".to_string(),
            msg: self.to_string(),
        }
    }
}

impl From<RequestError> for RustException {
    fn from(value: RequestError) -> Self {
        RustException::CreateRequest(value.to_string())
    }
}
