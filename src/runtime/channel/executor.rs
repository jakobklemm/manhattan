//! # Executor

use crate::Error;
use std::collections::HashMap;

pub struct Executor<State, R> {
    pub state: State,
    active: HashMap<usize, (Box<dyn FnMut(&mut State, R) -> anyhow::Result<usize>>, usize)>,
}

impl<S, R> Executor<S, R> {
    pub fn new(state: S) -> Self {
        Self {
            active: HashMap::new(),
            state,
        }
    }

    pub fn add<F>(&mut self, f: F, mid: usize, c: usize)
    where
        F: FnMut(&mut S, R) -> anyhow::Result<usize> + 'static,
    {
        self.active.insert(mid, (Box::new(f), c));
    }

    pub fn handle(&mut self, id: usize, message: R) -> Result<(), Error> {
        if let Some((f, c)) = self.active.get_mut(&id) {
            match (f)(&mut self.state, message) {
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
