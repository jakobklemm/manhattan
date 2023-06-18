//! # Identification
//!
//! To be used in the Router and Messages

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct ID {
    actor: u64,
    message: u64,
}

impl ID {
    pub fn new(actor: u64, message: u64) -> Self {
        Self { actor, message }
    }
}

impl Default for ID {
    fn default() -> Self {
        Self {
            actor: 0,
            message: 0,
        }
    }
}
