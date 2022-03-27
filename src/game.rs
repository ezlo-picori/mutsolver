/// Description of a game current state
use crate::Dict;

#[derive(Debug)]
pub enum GameError {
    InvalidSize(usize, usize, String), // expected size, found size, incriminated word
}
impl std::error::Error for GameError {}

impl std::fmt::Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InvalidSize(expected, found, word) => write!(
                f,
                "Size of '{}' differs from expectation ({} != {})",
                &word, &expected, &found
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum State {
    YES, // letter is correctly placed (RED)
    MEH, // letter is incorrectly placed (YELLOW)
    NO,  // letter is not in answer (BLUE)
}
pub type States = Vec<State>;

pub struct Attempt<'a>(pub &'a str, pub States);
pub type Attempts<'a> = Vec<Attempt<'a>>;

impl<'a, 'b> Attempt<'a> {
    pub fn from_answer(attempt: &'a str, answer: &'b str) -> Result<Self, GameError> {
        use std::collections::HashMap;

        let size = answer.chars().count();
        if attempt.chars().count() != size {
            return Err(GameError::InvalidSize(
                size,
                attempt.chars().count(),
                attempt.to_owned(),
            ));
        }
        let mut states = vec![State::NO; size];

        answer
            .chars()
            .zip(attempt.chars())
            .zip(states.iter_mut())
            .filter(|((ans, att), _)| *ans == *att)
            .for_each(|(_, sta)| *sta = State::YES);

        // Count not-identified characters
        let mut acc = answer
            .chars()
            .zip(states.iter())
            .filter(|(_, sta)| **sta != State::YES)
            .fold(HashMap::new(), |mut acc, (c, _)| {
                let count = acc.entry(c).or_insert(0);
                *count += 1;
                acc
            });

        for (sta, c) in states
            .iter_mut()
            .zip(attempt.chars())
            .filter(|(sta, _)| **sta != State::YES)
        {
            if let Some(count) = acc.get_mut(&c) {
                *sta = State::MEH;
                *count -= 1;
            }
        }

        Ok(Attempt(attempt, states))
    }
}

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
