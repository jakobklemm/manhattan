//! # Router

use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::{self, JoinHandle},
};

use dashmap::DashMap;

mod message;
pub use message::{Event, Message, Metadata};

mod id;
pub use id::ID;

pub struct Router {
    counter: u64,
    queue: VecDeque<Event>,
    receiver: Receiver<Event>,
    active: Arc<DashMap<u64, Sender<Event>>>,
}

impl Router {
    pub fn new(receiver: Receiver<Event>, active: Arc<DashMap<u64, Sender<Event>>>) -> Self {
        Self {
            counter: 0,
            queue: VecDeque::new(),
            receiver,
            active,
        }
    }

    pub fn run(self) -> JoinHandle<()> {
        task::spawn(async move {
            let mut router = self;
            loop {
                if let Some(event) = router.receiver.recv().await {
                    router.counter += 1;
                    {
                        if event.is_system() {
                            return;
                        }
                        let dest;
                        if event.is_message() {
                            dest = event.destination();
                        } else {
                            dest = event.source();
                        }
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

    #[derive(Debug)]
    struct Msg(u64);

    impl Message for Msg {}

    #[test]
    fn test_router_create() {
        // TODO: channel buffer size
        let (_tx, rx) = mpsc::channel(42);
        let active = Arc::new(DashMap::new());
        let r = Router::new(rx, active);
        assert_eq!(r.size(), 0);
    }

    #[tokio::test]
    async fn test_router_simple() {
        // TODO: channel buffer size
        let (tx, rx) = mpsc::channel(42);
        let a = Arc::new(DashMap::new());
        let r = Router::new(rx, a.clone());
        let h = r.run();

        let (s_a, mut r_a) = mpsc::channel(42);
        let (s_b, mut r_b) = mpsc::channel(42);

        let _ = a.insert(1, s_a);
        let _ = a.insert(2, s_b);

        // Send from "1" to "2"
        let meta_a = Metadata::new(1, 2, 0);
        let message = Event::new(meta_a, Box::new(Msg(42)));
        let _ = tx.send(message).await;

        // Receive at "2"
        let received = r_b.recv().await.unwrap();
        assert_eq!(received.source(), 1);
        assert_eq!(received.destination(), 2);
        assert!(received.is_message());

        // let body = received.body().unwrap();
        let reply = received.reply(Box::new(Msg(43)));
        let _ = tx.send(reply).await;

        // Receive at "1"
        let completed = r_a.recv().await.unwrap();
        assert!(!completed.is_message());

        if let Event::Reply(_, msg) = completed {
            assert_eq!(format!("{:?}", msg), format!("{:?}", Msg(43)))
        }

        let term = Event::shutdown();
        let _ = tx.send(term).await;
        let _ = h.await;
    }
}
