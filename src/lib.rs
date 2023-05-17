mod arguments;
mod error;
pub mod runtime;

use std::fmt::Display;

pub use arguments::Arguments;
pub use error::Error;

/// Test + Error => TError 
#[derive(Debug)]
struct TError(dyn std::error::Error);

impl Display for TError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl std::error::Error for TError {}

pub fn attempt() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
