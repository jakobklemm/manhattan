//! # Local Registry

use crate::runtime::{actors::Actor, Registry};
use crate::system::eid::EID;
use crate::Error;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct LocalRegistry(Arc<Mutex<HashMap<PID, EID>>>);

impl LocalRegistry {
    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}

impl Registry for LocalRegistry {
    type PID = PID;

    fn register(&self, _actor: impl Actor) -> Result<Self::PID, Error> {
        // TODO: System get Entity ID
        // TODO: Use actor?
        let eid = EID::default();
        let pid = PID::new();

        let mut hm = self.0.lock()?;

        hm.insert(pid.clone(), eid);

        Ok(pid)
    }

    fn lookup(&self, pid: &Self::PID) -> Option<EID> {
        if let Ok(hm) = self.0.lock() {
            if let Some(&ref eid) = hm.get(pid) {
                return Some(eid.clone());
            }
        }
        None
    }

    fn remove(&self, pid: &Self::PID) -> Option<EID> {
        if let Ok(mut hm) = self.0.lock() {
            hm.remove(pid)
        } else {
            None
        }
    }
}

#[derive(Hash, Debug, Clone, Eq, PartialEq)]
pub struct PID(usize);

use rand::prelude::*;

impl PID {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let id: usize = rng.gen();
        PID(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ActorA {}
    struct ActorB {}

    impl Actor for ActorA {
        fn retire(&self) -> Result<(), Error> {
            todo!()
        }

        fn source(&self) -> Result<(), Error> {
            todo!()
        }
    }

    impl Actor for ActorB {
        fn retire(&self) -> Result<(), Error> {
            todo!()
        }

        fn source(&self) -> Result<(), Error> {
            todo!()
        }
    }

    #[test]
    fn test_local_registry_construction() {
        let lr = LocalRegistry::new();

        let aa = ActorA {};
        let ab = ActorB {};

        let pa = lr.register(aa).unwrap();
        let pb = lr.register(ab).unwrap();

        assert_eq!(lr.remove(&pa).unwrap(), EID(0));
        assert_eq!(lr.remove(&pb).unwrap(), EID(1));
    }
}
