//! # Actors

use crate::Error;
use async_trait::async_trait;

#[async_trait]
pub trait Actor {
    async fn retire(&self) -> Result<(), Error>;
    async fn source(&self) -> Result<(), Error>;
}

#[async_trait]
pub trait Handle<T> {
    async fn handle(message: T) -> Result<(), Error>;
}
