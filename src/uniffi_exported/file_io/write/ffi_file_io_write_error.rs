use crate::prelude::*;
use thiserror::Error as ThisError;

#[derive(Debug, PartialEq, Eq, Clone, Error, ThisError)]
pub enum FFIFileIOWriteError {
    #[error("UnknownError: '{underlying}'")]
    Unknown { underlying: String },
}
