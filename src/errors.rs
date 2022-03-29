use crate::answers::Answer;
use crate::tests::Test;

#[derive(Debug)]
pub enum Error {
    IncompatibleAnswers(Answer, Answer),
    InvalidSize(usize, usize, String), // expected size, found size, incriminated word
    UnexpectedTest(Test, String),      // Invalid test, incriminated word
}
impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::IncompatibleAnswers(lhs, rhs) => write!(
                f,
                "Attempting to merge {:?} with {:?}: conflicting operation.",
                *lhs, *rhs,
            ),
            Self::InvalidSize(expected, found, word) => write!(
                f,
                "Size of '{}' ({}) differs from expectation ({})",
                &word, &expected, &found
            ),
            Self::UnexpectedTest(test, word) => {
                write!(f, "Test {:?} incompatible with word '{}'", test, word)
            }
        }
    }
}

#[derive(Debug)]
pub enum DictError {
    InconsistentSize(usize, usize, String), // expected size, found size, incriminated word
    DuplicateWord(usize, String),           // incriminated word count and value
    MissingAnswers,                         // Answer list is empty
    UnauthorizedCharacter(char, String),    // incriminated character and word
}

impl std::error::Error for DictError {}

impl std::fmt::Display for DictError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::InconsistentSize(expected, found, word) => write!(
                f,
                "Size of '{}' differs from expectation ({} != {})",
                &word, &expected, &found
            ),
            Self::DuplicateWord(count, word) => {
                write!(f, "Word '{}' found {} times.", &word, &count)
            }
            Self::MissingAnswers => {
                write!(f, "List of answers is empty.")
            }
            Self::UnauthorizedCharacter(character, word) => write!(
                f,
                "Word '{}' contains invalid character '{}'",
                &word, &character
            ),
        }
    }
}
