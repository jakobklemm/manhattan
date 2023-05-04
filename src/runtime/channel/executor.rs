//! # Executor

use crate::Error;
use std::collections::HashMap;

pub struct Executor<R, State> {
    active: HashMap<usize, (Box<dyn FnMut(&mut State, R) -> Result<usize, Error>>, usize)>,
}

impl<R, S> Executor<R, S> {
    pub fn new() -> Self {
        Self {
            active: HashMap::new(),
        }
    }

    pub fn add<F>(&mut self, f: F, mid: usize, c: usize)
    where
        F: FnMut(&mut S, R) -> Result<usize, Error> + 'static,
    {
        self.active.insert(mid, (Box::new(f), c));
    }

    pub fn handle(&mut self, s: &mut S, id: usize, message: R) -> Result<(), Error> {
        if let Some((f, c)) = self.active.get_mut(&id) {
            match (f)(s, message) {
                Ok(count) => {
                    *c = count;
                    return Ok(());
                }
                Err(..) => {
                    *c = 0;
                    return Err(Error::default());
                }
            }
        } else {
            return Err(Error::default());
        }
    }
}
