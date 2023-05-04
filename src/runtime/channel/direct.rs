//! # Direct Channel Communication

use super::executor::Executor;
use crate::Error;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub struct Connector<S, T> {
    sender: Sender<Event<T>>,
    receiver: Receiver<Event<T>>,
    pub executor: Executor<T, S>,
    counter: usize,
}

pub enum Event<T> {
    Message(usize, T),
    Reply(usize, T),
}

impl<S, T: Send + Sync + 'static> Connector<S, T> {
    pub fn new() -> (Self, Self) {
        let (s1, r1): (Sender<Event<T>>, Receiver<Event<T>>) = unbounded();
        let (s2, r2): (Sender<Event<T>>, Receiver<Event<T>>) = unbounded();
        let c1 = Connector {
            sender: s1,
            receiver: r2,
            executor: Executor::new(),
            counter: 0,
        };
        let c2 = Connector {
            sender: s2,
            receiver: r1,
            executor: Executor::new(),
            counter: 0,
        };

        (c1, c2)
    }

    pub fn call<F>(&mut self, message: T, f: F) -> anyhow::Result<()>
    where
        F: FnMut(&mut S, T) -> anyhow::Result<usize> + 'static,
    {
        let id = self.counter;
        self.counter += 1;
        let _ = self.sender.send(Event::Message(id, message))?;
        self.executor.add(f, id, 1);
        Ok(())
    }

    pub fn reply(&mut self, id: usize, message: T) -> anyhow::Result<()> {
        let _ = self.sender.send(Event::Reply(id, message))?;
        Ok(())
    }

    pub fn receive(&mut self, state: &mut S) -> anyhow::Result<(usize, T)> {
        let Ok(msg) = self.receiver.try_recv() else {
            return Err(anyhow::Error::new(Error::default()));
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

        let _ = c1.call(Message::Request, |ref mut state, message| {
            match message {
                Message::Request => {}
                Message::Send(i) => **state = i,
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
