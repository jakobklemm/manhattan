//! # Direct Channel Communication

use super::executor::Executor;
use crate::Error;
use std::fmt::Debug;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct Connector<S, T> {
    sender: UnboundedSender<Event<T>>,
    receiver: UnboundedReceiver<Event<T>>,
    pub executor: Executor<T, S>,
    counter: usize,
}

#[derive(Debug)]
pub enum Event<T> {
    Message(usize, T),
    Reply(usize, T),
}

impl<S, T> Connector<S, T>
where
    T: Send + Sync + Debug + 'static,
{
    pub fn new() -> (Self, Self) {
        let (s1, r1) = unbounded_channel();
        let (s2, r2) = unbounded_channel();
        let c1: Self = Connector {
            sender: s1,
            receiver: r2,
            executor: Executor::new(),
            counter: 0,
        };
        let c2: Self = Connector {
            sender: s2,
            receiver: r1,
            executor: Executor::new(),
            counter: 0,
        };

        (c1, c2)
    }

    pub fn call<F>(&mut self, message: T, f: F) -> Result<(), Error>
    where
        F: FnMut(&mut S, T) -> Result<usize, Error> + 'static,
    {
        let id = self.counter;
        self.counter += 1;
        let _ = self.sender.send(Event::Message(id, message))?;
        self.executor.add(f, id, 1);
        Ok(())
    }

    pub fn reply(&mut self, id: usize, message: T) -> Result<(), Error> {
        let _ = self.sender.send(Event::Reply(id, message))?;
        Ok(())
    }

    pub fn receive(&mut self, state: &mut S) -> Result<(usize, T), Error> {
        let Ok(msg) = self.receiver.try_recv() else {
            return Err(Error::default());
        };

        match msg {
            Event::Message(u, m) => Ok((u, m)),
            Event::Reply(u, m) => {
                self.executor.handle(state, u, m)?;
                self.receive(state)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum Message {
        Request,
        Send(usize),
    }

    #[test]
    fn test_basic_message_exchange() {
        let (mut c1, mut c2) = Connector::new();

        let mut state1 = 0;
        let mut state2 = 42;

        let _ = c1.call(Message::Request, |state, message| {
            match message {
                Message::Request => {}
                Message::Send(i) => *state = i,
            }
            Ok(0)
        });

        if let Ok((c, _msg)) = c2.receive(&mut state2) {
            let _ = c2.reply(c, Message::Send(state2));
        }

        let _ = c1.receive(&mut state1);

        assert_eq!(state1, 42);
    }
}
