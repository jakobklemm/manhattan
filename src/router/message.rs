//! # Message
///
/// Different message & event related stuff
use std::fmt::Debug;

use super::ID;

pub trait Message: Debug + Send + 'static {}

#[derive(Debug)]
pub enum Event<T: Message> {
    Message(Metadata, T),
    Reply(Metadata, T),
    System(SystemEvent),
}

#[derive(Debug, Copy, Clone)]
pub struct Metadata {
    id: ID,
    source: u64,
    destination: u64,
}

impl Metadata {
    pub(crate) fn new(local: u64, remote: u64, message: u64) -> Self {
        let id = ID::new(local, message);
        Self {
            id,
            source: local,
            destination: remote,
        }
    }

    fn source(&self) -> u64 {
        self.source
    }

    fn destination(&self) -> u64 {
        self.destination
    }

    fn identification(&self) -> ID {
        self.id
    }
}

#[derive(Debug)]
pub enum SystemEvent {
    Shutdown,
}

impl<T: Message> Event<T> {
    pub(crate) fn new(meta: Metadata, msg: T) -> Self {
        Self::Message(meta, msg)
    }

    pub(crate) fn shutdown() -> Self {
        Self::System(SystemEvent::Shutdown)
    }

    pub(crate) fn reply(&self, data: T) -> Self {
        match self {
            Self::Message(meta, _msg) => {
                // TODO: Maybe using &mut self?
                Self::Reply(*meta, data)
            }
            Self::Reply(_, _) => {
                // TODO: Change default or add error type
                Self::System(SystemEvent::Shutdown)
            }
            Self::System(_) => {
                // TODO: Change default or add error type
                Self::System(SystemEvent::Shutdown)
            }
        }
    }

    pub(crate) fn body(&self) -> Option<&T> {
        match self {
            Self::Message(_meta, msg) => Some(msg),
            Self::Reply(_meta, msg) => Some(msg),
            Self::System(_) => None,
        }
    }

    pub(crate) fn source(&self) -> u64 {
        match self {
            Self::Message(meta, _msg) => meta.source(),
            Self::Reply(meta, _msg) => meta.source(),
            Self::System(_) => 0,
        }
    }

    pub(crate) fn destination(&self) -> u64 {
        match self {
            Self::Message(meta, _msg) => meta.destination(),
            Self::Reply(meta, _msg) => meta.destination(),
            Self::System(_) => 0,
        }
    }

    pub(crate) fn identification(&self) -> ID {
        match self {
            Self::Message(id, _msg) => id.identification(),
            Self::Reply(id, _msg) => id.identification(),
            Self::System(_) => ID::default(),
        }
    }

    pub(crate) fn is_message(&self) -> bool {
        match self {
            Self::Message(_id, _msg) => true,
            Self::Reply(_id, _msg) => false,
            Self::System(_) => false,
        }
    }

    pub(crate) fn is_system(&self) -> bool {
        match self {
            Self::Message(_id, _msg) => false,
            Self::Reply(_id, _msg) => false,
            Self::System(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Msg {}
    impl Message for Msg {}

    #[test]
    fn test_create_event_simple() {
        let meta = Metadata::new(1, 2, 0);
        let e = Event::new(meta, Msg {});

        assert_eq!(e.source(), 1);
        assert_eq!(e.destination(), 2);
    }
}
