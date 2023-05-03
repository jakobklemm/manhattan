//! # Direct Channel Communication

use crossbeam_channel::{unbounded, Receiver, Sender};
use super::executor::Executor;

pub struct Connector<State: Sync, T: Send> {
    sender: Sender<(usize, T)>,
    receiver: Receiver<(usize, T)>,
    executor: Executor<State, T>,
    counter: usize,
}

impl<S: Sync, T: Send + Sync + 'static> Connector<S, T> {
    pub fn new(st1: S, st2: S) -> (Self, Self) {
        let (s1, r1): (Sender<(usize, T)>, Receiver<(usize, T)>) = unbounded();
        let (s2, r2): (Sender<(usize, T)>, Receiver<(usize, T)>) = unbounded();
        let c1 = Connector {
            sender: s1,
            receiver: r2.clone(),
            executor: Executor::new(st1), 
            counter: 0,
        };
        let c2 = Connector {
            sender: s2, 
            receiver: r1.clone(),
            executor: Executor::new(st2), 
            counter: 0,
        };
        (c1, c2)
    }

    pub fn call<F>(&mut self, message: T, f: F) -> anyhow::Result<()> 
        where F: Fn(&S, &T) -> anyhow::Result<usize> + 'static
    {
        let id = self.counter;
        self.counter += 1;
        let _ = self.sender.send((id, message))?; 
        self.executor.add(f, id, 1); 
        Ok(())
    }

    pub fn receive(&self) -> anyhow::Result<T> {
        let Ok((_id, msg)) = self.receiver.recv() else {
            return Err(anyhow::Error::new(super::executor::Error {}));
        };

        Ok(msg)
    }
}

