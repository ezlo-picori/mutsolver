use crate::answers::{Answer, Answers};
use crate::attempt::{Attempt, Attempts};
use crate::errors::Error;
use crate::options::Options;
use crate::tests::{Test, TestSuite};
use crate::Dict;

/// Description of a game current state

pub struct Game<'a> {
    attempts: Attempts<'a>,
    dict: &'a Dict,
    tests: TestSuite,
}

impl<'a> Game<'a> {
    pub fn new(dict: &'a Dict, options: &'_ Options) -> Self {
        Game {
            dict,
            attempts: Vec::new(),
            tests: Test::for_dict(dict, options),
        }
    }

    #[doc(hidden)]
    pub fn new_with_tests(dict: &'a Dict, tests: TestSuite) -> Self {
        Game {
            attempts: Vec::new(),
            dict,
            tests,
        }
    }

    pub fn add(&mut self, attempt: Attempt<'a>) {
        // TODO: check attempted word is in dict
        self.attempts.push(attempt);
    }

    pub fn dict(&self) -> &Dict {
        self.dict
    }

    pub fn attempts(&self) -> &Attempts {
        &self.attempts
    }

    /// Compute the answer of each test given known attempts.
    pub fn known_answers(&self) -> Result<Answers, Error> {
        self.tests
            .iter()
            .map(|test| {
                self.attempts
                    .iter()
                    .map(|attempt| attempt.answers(test))
                    .fold(Ok(Answer::Unknown), |acc, answer| match acc {
                        Err(_) => acc,
                        Ok(prev) => match answer {
                            Err(_) => answer,
                            Ok(new) => prev + new,
                        },
                    })
            })
            .collect()
    }
}
