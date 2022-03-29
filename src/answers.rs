use crate::errors::Error;
use crate::{Dict, TestSuite};

/// Structure containing answers for a test suite
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Answer {
    Unknown,
    Yes,
    No,
}

impl std::ops::Add<Self> for Answer {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Unknown => Ok(rhs),
            Self::Yes => match rhs {
                Self::Unknown => Ok(self),
                Self::Yes => Ok(self),
                Self::No => Err(Error::IncompatibleAnswers(self, rhs)),
            },
            Self::No => match rhs {
                Self::Unknown => Ok(self),
                Self::Yes => Err(Error::IncompatibleAnswers(self, rhs)),
                Self::No => Ok(self),
            },
        }
    }
}

// TODO
// Implement Answer addition => X + X = Ok(X)
//                           => Unknown + X = Ok(X)
//                           => Yes + No = Err(InconsistentAnswers)
// On a game, compute answers for each attempt and merge these
// answers using this addition rule. This gives:
//  * which tests have been answered
//  * what is the answer of these tests for the game solution
// NB: HasPrefix/Suffix can be answered by combining multiple attempts
//     even if no single attempt answered the test. Think of a way to
//     postprocess the list of answers to answer Unknown tests by
//     combining results from other resolved tests.

#[derive(Debug, PartialEq)]
pub struct Answers(pub Vec<Answer>);

type DictAnswers = Vec<Answers>;

impl std::iter::FromIterator<Answer> for Answers {
    fn from_iter<I: std::iter::IntoIterator<Item = Answer>>(iter: I) -> Self {
        Self(Vec::from_iter(iter))
    }
}

impl Answers {
    /// Generate answers for all tests in a test-suite for a given word.
    pub fn of_word(word: &str, tests: &TestSuite) -> Self {
        tests
            .iter()
            .map(|test| match test.run(word) {
                true => Answer::Yes,
                false => Answer::No,
            })
            .collect()
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

impl std::ops::Add<Self> for Answers {
    type Output = Result<Self, Error>;

    fn add(self, rhs: Self) -> Self::Output {
        self.0
            .iter()
            .zip(rhs.0.iter())
            .map(|(l, r)| *l + *r)
            .collect()
    }
}
