//! # Router

use std::collections::{HashMap, VecDeque};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::{self, JoinHandle},
};

mod message;
pub use message::{Event, Message};

pub struct Router {
    counter: u64,
    queue: VecDeque<Event>,
    receiver: Receiver<Box<dyn Message>>,
    active: HashMap<u64, Sender<Event>>,
    pending: HashMap<u64, u64>,
}

impl Router {
    pub fn new(receiver: Receiver<Box<dyn Message>>) -> Self {
        Self {
            counter: 0,
            queue: VecDeque::new(),
            receiver,
            active: HashMap::new(),
            pending: HashMap::new(),
        }
    }

    pub fn run(self) -> JoinHandle<()> {
        task::spawn(async move {
            let mut router = self;
            loop {
                if let Some(message) = router.receiver.recv().await {
                    router.counter += 1;
                    {
                        let event = Event::new(router.counter, message);
                        let dest = event.destination();
                        if let Some(sndr) = router.active.get(&dest) {
                            // TODO: Handle send error
                            let _ = sndr.send(event).await;
                        } else {
                            router.queue.push_back(event);
                        }
                    }
                }
            }
        })
    }

    pub fn size(&self) -> usize {
        self.queue.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_router() {
        // TODO: channel buffer size
        let (_tx, rx) = mpsc::channel(42);
        let r = Router::new(rx);
        assert_eq!(r.size(), 0);
    }
}
