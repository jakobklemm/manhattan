//! # Direct Channel Communication

use super::executor::Executor;
use crate::Error;
use crossbeam_channel::{unbounded, Receiver, Sender};

pub struct Connector<State, T> {
    sender: Sender<Event<T>>,
    receiver: Receiver<Event<T>>,
    pub executor: Executor<State, T>,
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

    pub fn receive(&mut self) -> anyhow::Result<(usize, T)> {
        let Ok(msg) = self.receiver.try_recv() else {
            return Err(anyhow::Error::new(Error::default()));
        };

        match msg {
            Event::Message(u, m) => Ok((u, m)),
            Event::Reply(u, m) => {
                self.executor.handle(u, m)?;
                self.receive() 
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
        Send(usize)
    }

    #[test]
    fn test_basic_message_exchange() {
        let (mut c1, mut c2) = Connector::new(0, 0);

        let _ = c1.call(Message::Request, |ref mut state, message| {
            println!("c1 received: {:?}", message);
            match message {
                Message::Request => {},
                Message::Send(i) => **state = i
            }
            Ok(0)
        });

        if let Ok((c, msg)) = c2.receive() {
            println!("c2 received: {:?}", msg);
            let _ = c2.reply(c, Message::Send(42));
        }

        let _ = c1.receive();

        let t = std::time::Duration::from_millis(100);
        std::thread::sleep(t);

        assert_eq!(c1.executor.state, 42);
    }
}
