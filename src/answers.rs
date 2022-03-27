use crate::{Dict, TestSuite};
/// Structure containing answers for a test suite
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq)]
pub struct Answers(pub Vec<bool>);

type DictAnswers = Vec<Answers>;

impl Answers {
    /// Generate answers for all tests in a test-suite for a given word.
    pub fn of_word(word: &str, tests: &TestSuite) -> Self {
        let mut answers = Answers(Vec::with_capacity(tests.len()));

        tests.iter().for_each(|test| answers.0.push(test.run(word)));

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
}
