mod error;
pub mod runtime;

use std::fmt::Display;

pub use error::Error;

/// Test + Error => TError
#[derive(Debug)]
struct TError(Option<Box<dyn std::error::Error>>);

impl Display for TError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for TError {}

pub fn attempt() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

pub fn another() -> Result<(), impl std::error::Error> {
    let e = TError(None);

    Err(e)
}
