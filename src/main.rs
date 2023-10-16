use std::{any::Any, sync::mpsc::channel};

pub trait Handle<M> {
    fn handle(message: M);
}

pub struct Actor {}
pub struct Message {}
pub struct Action {}

impl Handle<Message> for Actor {
    fn handle(_message: Message) {
        println!("Handled message");
    }
}

impl Handle<Action> for Actor {
    fn handle(_message: Action) {
        println!("Handled action");
    }
}

pub struct Envelope {
    inner: Box<dyn Any>,
}

impl Envelope {
    fn new(msg: Box<dyn Any>) -> Self {
        Self { inner: msg }
    }

    fn inner<M>(&self) -> Option<M> {
        match self.inner.downcast() {
            Ok(inr) => Some(*inr),
            Err(_) => None,
        }
    }
}

fn main() {
    println!("test");
    let actor = Actor {};
    let (sender, receiver) = channel();
    let msg = Envelope::new(Box::new(Message {}));
    let _ = sender.send(msg);

    // receiver side
    let back = receiver.recv().unwrap();
    let typed = back.inner().unwrap();
    Actor::handle(typed);
}
