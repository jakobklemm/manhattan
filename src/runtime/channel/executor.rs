//! # Executor 

use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Error {}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error")
    }
}

pub struct Executor<State: Sync, R> {
    state: State,
    active: HashMap<usize, (Box<dyn Fn(&State, &R) -> anyhow::Result<usize>>, usize)>
}

impl<S: Sync, R> Executor<S, R> {
    pub fn new(state: S) -> Self {
        Self {
            active: HashMap::new(),
            state
        }
    }

    pub fn add<F>(&mut self, f: F, mid: usize, c: usize) 
        where F: Fn(&S, &R) -> anyhow::Result<usize> + 'static 
    {
       self.active.insert(mid, (Box::new(f), c)); 
    }

    pub fn handle(&mut self, id: usize, message: &R) -> Result<(), Error> {
        if let Some((f, c)) = self.active.get_mut(&id) {
            match (f)(&self.state, message) {
                Ok(count) => {
                    *c = count;
                    return Ok(())
                }
                Err(..) => {
                    *c = 0;
                    return Err(Error {})
                }
            }
        } else {
            return Err(Error {})
        }
    }
}
