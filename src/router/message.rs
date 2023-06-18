//! # Message
///
/// Different message & event related stuff
use std::fmt::Debug;

use super::ID;

/// TODO: Change u64 to identity type.
pub trait Message: Debug + Send {
    fn source(&self) -> u64;
    fn destination(&self) -> u64;
}

pub type MsgImpl = Box<dyn Message>;

#[derive(Debug)]
pub enum Event {
    Message(ID, MsgImpl),
    Reply(ID, MsgImpl),
    System(SystemEvent),
}

#[derive(Debug)]
pub enum SystemEvent {
    Shutdown,
}

impl Event {
    pub(crate) fn new(id: ID, msg: MsgImpl) -> Self {
        Self::Message(id, msg)
    }

    pub(crate) fn shutdown() -> Self {
        Self::System(SystemEvent::Shutdown)
    }

    pub(crate) fn reply(&self, data: MsgImpl) -> Self {
        match self {
            Self::Message(id, _msg) => {
                // TODO: Maybe using &mut self?
                Self::Reply(*id, data)
            }
            Self::Reply(_id, _msg) => {
                // TODO: Change default or add error type
                Self::System(SystemEvent::Shutdown)
            }
            Self::System(_) => {
                // TODO: Change default or add error type
                Self::System(SystemEvent::Shutdown)
            }
        }
    }

    pub(crate) fn source(&self) -> u64 {
        match self {
            Self::Message(_id, msg) => msg.source(),
            Self::Reply(_id, msg) => msg.source(),
            Self::System(_) => 0,
        }
    }

    pub(crate) fn destination(&self) -> u64 {
        match self {
            Self::Message(_id, msg) => msg.destination(),
            Self::Reply(_id, msg) => msg.destination(),
            Self::System(_) => 0,
        }
    }

    pub(crate) fn id(&self) -> ID {
        match self {
            Self::Message(id, _msg) => *id,
            Self::Reply(id, _msg) => *id,
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

    pub(crate) fn is_reply(&self) -> bool {
        match self {
            Self::Message(_id, _msg) => false,
            Self::Reply(_id, _msg) => true,
            Self::System(_) => false,
        }
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
        let id = ID::new(1, 1);
        let e = Event::new(id, Box::new(Msg {}));

        assert_eq!(e.source(), 0);
        assert_eq!(e.destination(), 1);
    }
}
