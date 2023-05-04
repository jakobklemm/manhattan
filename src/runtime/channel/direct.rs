//! # Direct Channel Communication

use super::executor::Executor;
use crate::Error;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub struct Connector<State, T> {
    sender: Sender<Event<T>>,
    receiver: Receiver<Event<T>>,
    executor: Executor<State, T>,
    counter: usize,
}

pub enum Event<T> {
    Message(usize, T),
    Reply(usize, T),
}

impl<S: Sync, T: Send + Sync + 'static> Connector<S, T> {
    pub fn new(st1: S, st2: S) -> (Self, Self) {
        let (s1, r1): (Sender<Event<T>>, Receiver<Event<T>>) = unbounded();
        let (s2, r2): (Sender<Event<T>>, Receiver<Event<T>>) = unbounded();
        let c1 = Connector {
            sender: s1,
            receiver: r2,
            executor: Executor::new(st1),
            counter: 0,
        };
        let c2 = Connector {
            sender: s2,
            receiver: r1,
            executor: Executor::new(st2),
            counter: 0,
        };

        (c1, c2)
    }

    pub fn call<F>(&mut self, message: T, f: F) -> anyhow::Result<()>
    where
        F: Fn(&S, &T) -> anyhow::Result<usize> + 'static,
    {
        let id = self.counter;
        self.counter += 1;
        let _ = self.sender.send(Event::Message(id, message))?;
        self.executor.add(f, id, 1);
        Ok(())
    }

    pub fn receive(&mut self) -> anyhow::Result<Event<T>> {
        let Ok(msg) = self.receiver.recv() else {
            return Err(anyhow::Error::new(Error::default()));
        };
        Ok(msg)
    }
}
