use crate::answers::{Answer, Answers, DictAnswers};
use crate::attempt::{Attempt, Attempts};
use crate::errors::Error;
use crate::options::Options;
use crate::tests::{Test, TestSuite};
use crate::Dict;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// Description of a game current state

pub struct Game<'a> {
    attempts: Attempts<'a>,
    dict: &'a Dict,
    dict_answers: DictAnswers,
    tests: TestSuite,
}

#[derive(Debug, PartialEq)]
pub enum Guess {
    Solution(String),  // Word solution
    Candidate(String), // Candidate for next attempt and its likelyhood
    Sacrifice(String), // Known wrong attempt but help identifying the solution
    NoSolution,        // No word matches current game state
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
    pub fn guess_next(&self) -> Result<Guess, Error> {
        // 1 - Compute current answers
        let known_answers = self.known_answers()?;

        // 2 - Filter dict "answer" words to keep only ones compatibles with
        // current answers
        let compatible_words: Vec<(&String, &Answers)> = self
            .dict
            .answers
            .iter()
            .zip(self.dict_answers.iter())
            .filter(|(_, word_answers)| {
                known_answers
                    .iter()
                    .zip(word_answers.iter())
                    .all(|(known_answer, word_answer)| (*known_answer + *word_answer).is_ok())
            })
            .collect();

        // 3 - Count compatible words (N)
        match compatible_words.len() {
            0 => Ok(Guess::NoSolution),
            1 => Ok(Guess::Solution(compatible_words[0].0.clone())),
            n => {
                let is_answered: Vec<bool> = known_answers
                    .iter()
                    .map(|answer| *answer != Answer::Unknown)
                    .collect();

                // 4 - For each unknown test, count the number of compatible words which answer Yes (n)
                let test_positive_count = compatible_words
                    .iter()
                    .map(|(_, answers)| {
                        answers
                            .iter()
                            .zip(is_answered.iter())
                            .map(|(answer, ignore)| {
                                if !*ignore && *answer == Answer::Yes {
                                    1
                                } else {
                                    0
                                }
                            })
                            .collect::<Vec<u64>>()
                    })
                    .reduce(|count_a, count_b| {
                        count_a
                            .iter()
                            .zip(count_b.iter())
                            .map(|(a, b)| a + b)
                            .collect()
                    })
                    .unwrap();

                // 5 - For each unknown test, give a weight w = - n ( n - N )
                //     This law give the highest weight to tests which partition compatible answers in half
                let test_weight: Vec<u64> = test_positive_count
                    .iter()
                    .map(|c| c * (n as u64 - c))
                    .collect();

                // 6 - Iterate over all words (answer + allowed) and compute for each its own score by
                //     iterating over all tests and for each:
                //     - compute the probability "p" that this word will answer the test
                //     - add to the word score: s += p * w
                let word_score: Vec<u64> = self
                    .dict_answers
                    .par_iter()
                    .map(|_word_answers| {
                        // Compute list of probabilities for this word
                        self.tests
                            .iter()
                            .map(|test| match test {
                                Test::At(_, _) => 1,
                                _ => 26,
                            })
                            .zip(test_weight.iter())
                            .map(|(probability, weight)| probability * *weight)
                            .sum()
                    })
                    .collect();

                let best_candidate = self
                    .dict
                    .answers
                    .iter()
                    .zip(word_score.iter())
                    .max_by_key(|(_, score)| **score)
                    .unwrap();

                let best_sacrifice = self
                    .dict
                    .allowed
                    .iter()
                    .zip(word_score.iter().skip(self.dict.answers.len() + 1))
                    .max_by_key(|(_, score)| **score);

                match best_sacrifice {
                    Some((word, score)) if score > best_candidate.1 => {
                        Ok(Guess::Sacrifice(word.clone()))
                    }
                    None | Some(_) => Ok(Guess::Candidate(best_candidate.0.clone())),
                }
            }
        }
    }
}
