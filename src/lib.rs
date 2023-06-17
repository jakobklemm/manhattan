#![feature(async_fn_in_trait)]

mod router;

pub use router::Router;

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
    async fn handle(&self, message: TMessage, _state: Box<dyn State>, context: Context) {
        match message {
            TMessage::Ping => {
                let _ = context.sender.send(Box::new(TMessage::Pong));
            }
            TMessage::Pong => {
                let _ = context.sender.send(Box::new(TMessage::Ping));
            }
        }
    }
}

use std::sync::mpsc::Sender;
