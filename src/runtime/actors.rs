//! # Actors

use crate::Error;
// TODO: Async
// use async_trait::async_trait;

pub trait Actor {
    fn retire(&self) -> Result<(), Error>;
    fn source(&self) -> Result<(), Error>;
}

pub trait Handle<T> {
    fn handle(message: T) -> Result<(), Error>;
}
