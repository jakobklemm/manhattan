//! # EID - Entity ID

use std::sync::atomic::{AtomicU64, Ordering};

static PROCESS_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct EID(pub u64);

impl Default for EID {
    fn default() -> Self {
        Self(PROCESS_ID.fetch_add(1, Ordering::SeqCst))
    }
}
