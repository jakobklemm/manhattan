//! # Channel
//!
//! (Probably) very bad sinlge producer single consumer channel.
//!

use std::sync::atomic::AtomicUsize;

pub struct Sender<T> {
    inner: Inner<T>,
}
pub struct Receiver<T> {
    inner: Inner<T>,
}

struct Inner<T> {
    queue: Box<[T]>,
    tail: AtomicUsize,
    head: AtomicUsize,
}
