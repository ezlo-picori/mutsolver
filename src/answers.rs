use crate::{Dict, TestSuite};
/// Structure containing answers for a test suite
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq)]
pub enum Answer {
    UNKNOWN,
    YES,
    NO,
}

// TODO
// Implement Answer addition => X + X = Ok(X)
//                           => UNKNOWN + X = Ok(X)
//                           => YES + NO = Err(InconsistentAnswers)
// On a game, compute answers for each attempt and merge these
// answers using this addition rule. This gives:
//  * which tests have been answered
//  * what is the answer of these tests for the game solution
// NB: HasPrefix/Suffix can be answered by combining multiple attempts
//     even if no single attempt answered the test. Think of a way to
//     postprocess the list of answers to answer UNKNOWN tests by
//     combining results from other resolved tests.

#[derive(Debug, PartialEq)]
pub struct Answers(pub Vec<Answer>);

type DictAnswers = Vec<Answers>;

impl Answers {
    /// Generate answers for all tests in a test-suite for a given word.
    pub fn of_word(word: &str, tests: &TestSuite) -> Self {
        let mut answers = Answers(Vec::with_capacity(tests.len()));

        tests.iter().for_each(|test| {
            answers.0.push(match test.run(word) {
                true => Answer::YES,
                false => Answer::NO,
            })
        });

        answers
    }

    /// Generate answers for all tests in a test-suite for each word of a dict.
    pub fn of_dict(dict: &Dict, tests: &TestSuite) -> DictAnswers {
        let mut dict_answers = DictAnswers::with_capacity(dict.len());

        dict.par_iter()
            .map(|word| Answers::of_word(word, tests))
            .collect_into_vec(&mut dict_answers);

        dict_answers
    }

    // /// Generate answers for a game given current attempts
    // pub fn of_game(game: &Game, tests: &TestSuite ) -> Answers {
    //     let mut answers = Answers(Vec::with_capacity(tests.len()));

    //     // TODO: finish after attempts.answers(&Test) -> Answer
    // }
}
