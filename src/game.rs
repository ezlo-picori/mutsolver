/// Description of a game current state
use crate::{Answer, Dict, Test};

#[derive(Debug)]
pub enum GameError {
    InvalidSize(usize, usize, String), // expected size, found size, incriminated word
    UnexpectedTest(Test, String),      // Invalid test, incriminated word
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
            Self::UnexpectedTest(test, word) => {
                write!(f, "Test {:?} incompatible with word '{}'", test, word)
            }
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

        // Traverse attempted incorrect characters and mark them MEH if
        // answer counter for this character is non-null (then decrement
        // said counter)
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

    pub fn answers(&self, test: &Test) -> Result<Answer, GameError> {
        let states = &self.1;
        match test {
            Test::At(tl, tp) => {
                // test letter, test position
                let letter =
                    self.0.chars().nth(*tp).ok_or_else(|| {
                        GameError::UnexpectedTest(test.clone(), self.0.to_owned())
                    })?;
                let state = states.get(*tp).unwrap();
                match state {
                    State::YES => {
                        if *tl == letter {
                            Ok(Answer::YES)
                        } else {
                            Ok(Answer::NO)
                        }
                    }
                    State::MEH | State::NO => {
                        if *tl == letter {
                            Ok(Answer::NO)
                        } else {
                            Ok(Answer::UNKNOWN)
                        }
                    }
                }
            }
            Test::HasAtMost(tl, tc) => {
                // test letter, test count
                let occurences = self.0.chars().filter(|l| *l == *tl).count();
                let valid_occurences = self
                    .0
                    .chars()
                    .zip(states.iter())
                    .filter(|(l, s)| *l == *tl && **s != State::NO)
                    .count();

                if occurences == valid_occurences {
                    Ok(Answer::UNKNOWN)
                } else if valid_occurences <= *tc as usize {
                    Ok(Answer::YES)
                } else {
                    Ok(Answer::NO)
                }
            }
            Test::HasAtLeast(tl, tc) => {
                // test letter, test count
                let letter_count = self.0.chars().filter(|l| *l == *tl).count();
                if letter_count < *tc as usize {
                    Ok(Answer::UNKNOWN)
                } else if self
                    .0
                    .chars()
                    .zip(states.iter())
                    .filter(|(l, s)| *l == *tl && **s != State::NO)
                    .count()
                    >= *tc as usize
                {
                    Ok(Answer::YES)
                } else {
                    Ok(Answer::NO)
                }
            }
            Test::HasPrefix(prefix) => {
                if prefix.len() > self.0.chars().count() {
                    Err(GameError::UnexpectedTest(test.clone(), self.0.to_owned()))
                } else if prefix
                    .chars()
                    .zip(self.0.chars())
                    .zip(self.1.iter())
                    .all(|((pc, ac), st)| pc == ac && *st == State::YES)
                {
                    Ok(Answer::YES)
                } else if prefix.chars().zip(self.0.chars()).zip(self.1.iter()).any(
                    |((pc, ac), st)| pc != ac && *st == State::YES || pc == ac && *st != State::YES,
                ) {
                    Ok(Answer::NO)
                } else {
                    Ok(Answer::UNKNOWN)
                }
            }
            Test::HasSuffix(suffix) => {
                if suffix.len() > self.0.chars().count() {
                    return Err(GameError::UnexpectedTest(test.clone(), self.0.to_owned()));
                }

                let start = self.0.chars().count() - suffix.len();

                if suffix
                    .chars()
                    .zip(self.0.chars().skip(start))
                    .zip(self.1.iter().skip(start))
                    .all(|((sc, ac), st)| sc == ac && *st == State::YES)
                {
                    Ok(Answer::YES)
                } else if suffix
                    .chars()
                    .zip(self.0.chars().skip(start))
                    .zip(self.1.iter().skip(start))
                    .any(|((sc, ac), st)| {
                        sc != ac && *st == State::YES || sc == ac && *st != State::YES
                    })
                {
                    Ok(Answer::NO)
                } else {
                    Ok(Answer::UNKNOWN)
                }
            }
        }
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
