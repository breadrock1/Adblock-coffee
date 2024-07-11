use jni::errors::{Error, Exception, ToException};

use adblock::request::RequestError;
use std::fmt::Debug;
use std::sync::PoisonError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RustException {
    #[error("Failed while creating request: {0}")]
    CreateRequest(String),
    #[error("Failed while extracting parameter: {0}")]
    ExtractParameter(String),
    #[error("Failed while lock mutex for AdvtBlocker: {0}")]
    MutexGuardLock(String),
    #[error("Jvm runtime error: {0}")]
    JvmException(String),
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

impl <T> From<PoisonError<T>> for RustException {
    fn from(value: PoisonError<T>) -> Self {
        RustException::MutexGuardLock(value.to_string())
    }
}

impl From<jni::errors::Error> for RustException {
    fn from(value: Error) -> Self {
        RustException::JvmException(value.to_string())
    }
}
