//! # EID - Entity ID
//!

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct EID {}

impl Default for EID {
    fn default() -> Self {
        Self {}
    }
}
