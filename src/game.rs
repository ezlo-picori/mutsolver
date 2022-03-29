use crate::attempt::{Attempt, Attempts};
/// Description of a game current state
use crate::Dict;

pub struct Game<'a> {
    dict: &'a Dict,
    attempts: Attempts<'a>,
}

impl<'a> Game<'a> {
    pub fn new(dict: &'a Dict) -> Self {
        Game {
            dict,
            attempts: Vec::new(),
        }
    }

    pub fn add(&mut self, attempt: Attempt<'a>) {
        self.attempts.push(attempt);
    }

    pub fn dict(&self) -> &Dict {
        self.dict
    }

    pub fn attempts(&self) -> &Attempts {
        &self.attempts
    }
}
