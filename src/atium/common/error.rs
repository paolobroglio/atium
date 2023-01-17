use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)]
/// This is a specific Error for Atium
pub enum AtiumError {
    ConversionError(String),
    IOError(String),
    CommandError(String)
}

impl fmt::Display for AtiumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            AtiumError::ConversionError(ref msg) => write!(f, "Conversion Error: {}", msg),
            AtiumError::IOError(ref msg) => write!(f, "I/O Error: {}", msg),
            AtiumError::CommandError(ref msg) => write!(f, "Command Error: {}", msg)
        }
    }
}