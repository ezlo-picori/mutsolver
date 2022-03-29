use crate::answers::{Answer, Answers, DictAnswers};
use crate::attempt::{Attempt, Attempts};
use crate::errors::Error;
use crate::options::Options;
use crate::tests::{Test, TestSuite};
use crate::Dict;

/// Description of a game current state

pub struct Game<'a> {
    attempts: Attempts<'a>,
    dict: &'a Dict,
    dict_answers: DictAnswers,
    tests: TestSuite,
}

pub enum Guess {
    Solution(String),       // Word solution
    Candidate(String, f32), // Candidate for next attempt and its likelyhood
    Sacrifice(String),      // Known wrong attempt but help identifying the solution
    NoSolution,             // No word matches current game state
}

impl<'a> Game<'a> {
    pub fn new(dict: &'a Dict, options: &'_ Options) -> Self {
        let tests = Test::for_dict(dict, options);
        Game {
            attempts: Vec::new(),
            dict,
            dict_answers: Answers::of_dict(dict, &tests),
            tests,
        }
    }

    #[doc(hidden)]
    pub fn new_with_tests(dict: &'a Dict, tests: TestSuite) -> Self {
        Game {
            attempts: Vec::new(),
            dict,
            dict_answers: Answers::of_dict(dict, &tests),
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

        // TODO: post-process answers to check if we can answer some more by combining
        // results from all attempts.
        // Example: answer unknown prefix/suffix if all associated AT tests were answered.
    }

    /// Compute the most relevant guess to attempt at next try
    pub fn guess_next(&self) -> Guess {
        // 1 - Compute current answers
        // let known_answers = self.known_answers();

        // 2 - Filter dict "answer" words to keep only ones compatibles with
        // current answers
        // let compatible_words = dict.answers().iter().filter().collect();

        // 3 - Count compatible words (N)
        // If 0 => Return NoSolution
        // If 1 => Return Solution(word)
        // Else continue

        // 4 - For each unknown test, count the number of compatible word which answer Yes (n)

        // 5 - For each unknown test, give a weight w = - n ( n - N )
        //     This law give the highest weight to tests which partition compatible answers in half

        // 6 - Iterate over all words (answer + allowed) and compute for each its own score by
        //     iterating over all tests and for each:
        //     - compute the probability "p" that this word will answer the test
        //     - add to the word score: s += p * w
        //
        //    Idea: first iterate over allowed words, the best one will be a sacrifice
        //          then iterate over answers, if any has a highest score than the sacrifice, then return
        //          this candidate
        //    Or compute and return both the sacrifice and the candidate.
        Guess::NoSolution
    }
}
