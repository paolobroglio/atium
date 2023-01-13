use std::error::Error;
use std::fmt;
use std::fmt::{Formatter, write};

#[derive(Debug)]
pub enum AtiumError {
    ConversionError(String),
    ThumbnailError(String),
    IOError(String),
    CommandError(String),
    Other
}

impl fmt::Display for AtiumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AtiumError::ConversionError(ref msg) => write!(f, "Conversion Error: {}", msg),
            AtiumError::IOError(ref msg) => write!(f, "I/O Error: {}", msg),
            AtiumError::CommandError(ref msg) => write!(f, "Command Error: {}", msg),
            AtiumError::ThumbnailError(ref msg) => write!(f, "Thumbnail Error: {}", msg),
            AtiumError::Other => write!(f, "Unknown Error"),
        }
    }
}