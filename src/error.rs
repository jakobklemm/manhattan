//! # Error

use std::fmt::{self, Debug, Display, Formatter};
use tokio::sync::mpsc::error::SendError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefaultError {}

impl std::error::Error for DefaultError {}

impl Display for DefaultError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "")
    }
}

use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Error {
    /// Default variant for unspecified errors
    Unknown(String),
    /// Channel failed
    Channel(String),
    /// Contains inner error test
    With(Rc<dyn std::error::Error>)
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

impl std::error::Error for Error {
   fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
       match self {
           Self::With(e) => {
               Some(e)
           }
           _ => None
       }
   } 
}

impl<T: Debug> From<SendError<T>> for Error {
    fn from(value: SendError<T>) -> Self {
        Self::Channel(format!("tokio channel send failed: {:?}", value))
    }
}
