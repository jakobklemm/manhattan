//! # Actors

use crate::Error;
// TODO: Async
// use async_trait::async_trait;
//
//

pub trait Actor {
    async fn retire(&self) -> Result<(), Error>;
    async fn source(&self) -> Result<(), Error>;
}

pub trait Handle<T> {
    async fn handle(message: T) -> Result<(), Error>;
}
