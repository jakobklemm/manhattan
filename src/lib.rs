#![feature(async_fn_in_trait)]

mod error;
pub mod runtime;

pub mod util;

mod store;
mod system;

pub use error::Error;

trait State {}

trait Actor {}

trait Message: std::fmt::Debug {}

struct Context {
    sender: Sender<Box<dyn Message>>,
}

trait Handle<M: Message> {
    async fn handle(&self, message: M, state: Box<dyn State>, context: Context);
}

#[derive(Clone, Debug)]
struct TActor {}

#[derive(Clone, Debug)]
struct TState {}

impl Actor for TActor {}

impl State for TState {}

impl Message for TMessage {}

#[derive(Debug)]
enum TMessage {
    Ping,
    Pong,
}

impl Handle<TMessage> for TActor {
    async fn handle(&self, message: TMessage, state: Box<dyn State>, context: Context) {
        match message {
            TMessage::Ping => {
                context.sender.send(Box::new(TMessage::Pong));
            }
            _ => {}
        }
    }
}

struct SystemMessage {
    id: String,
    message: Box<dyn Message>,
}

use std::sync::mpsc::{self, Receiver, Sender};
