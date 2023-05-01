//! # Error

use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Default variant for unspecified errors
    Unknown(String),
}

impl Default for Error {
    fn default() -> Self {
        Self::Unknown(String::from("unknown error occured."))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // TODO: change from debug printer
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}
