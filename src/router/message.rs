//! # Message
///
/// Different message & event related stuff
use std::fmt::Debug;

/// TODO: Change u64 to identity type.
pub trait Message: Debug + Send {
    fn source(&self) -> u64;
    fn destination(&self) -> u64;
}

#[derive(Debug)]
pub struct Event {
    id: u64,
    content: Box<dyn Message>,
}

impl Event {
    pub(crate) fn new(id: u64, content: Box<dyn Message>) -> Self {
        Self { id, content }
    }

    pub(crate) fn source(&self) -> u64 {
        self.content.source()
    }

    pub(crate) fn destination(&self) -> u64 {
        self.content.destination()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Msg {}
    impl Message for Msg {
        fn source(&self) -> u64 {
            0
        }

        fn destination(&self) -> u64 {
            1
        }
    }

    #[test]
    fn test_create_event_simple() {
        let e = Event::new(1, Box::new(Msg {}));

        assert_eq!(e.content.source(), 0);
        assert_eq!(e.content.destination(), 1);
    }
}
