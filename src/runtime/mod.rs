use self::actors::Actor;
use crate::{system::eid::EID, Error};

// TODO: Make async
// use async_trait::async_trait;

pub mod actors;
pub mod channel;

pub struct Runtime<R>
where
    R: Registry,
{
    pub registry: R,
}

impl<R> Runtime<R>
where
    R: Registry,
{
    pub fn new(registry: R) -> Self {
        Self { registry }
    }

    pub async fn register(_actor: impl Actor) -> Result<(), Error> {
        Ok(())
    }
}

pub trait Registry: Sync {
    type PID;

    // actor dependent pid maybe
    fn register(&self, actor: impl Actor) -> Result<Self::PID, Error>;
    fn lookup(&self, pid: &Self::PID) -> Option<EID>;
    fn remove(&self, pid: &Self::PID) -> Option<EID>;
}
