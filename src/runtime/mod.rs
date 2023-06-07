use self::actors::Actor;
use crate::Error;

use async_trait::async_trait;

pub mod actors;
pub mod channel;
pub mod pid;

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

#[async_trait]
pub trait Registry {
    type PID;

    async fn register(actor: impl Actor) -> Self::PID;
}
