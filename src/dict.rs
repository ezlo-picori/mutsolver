use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
pub enum DictError {
    InconsistentSize(usize, usize, String), // expected size, found size, incriminated word
    DuplicateWord(usize, String),           // incriminated word count and value
    MissingAnswers,                         // Answer list is empty
    UnauthorizedCharacter(char, String),    // incriminated character and word
}

impl Error for DictError {}

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

pub type WordList = Vec<String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Dict {
    /// List of possible answers
    pub answers: WordList,
    /// List of words that can be submitted but won't be the answer
    #[serde(default)]
    pub allowed: WordList,
    /// Size of words
    size: usize,
}

// Implement constructor for Dict
impl Dict {
    pub fn new(answers: WordList, allowed: WordList) -> Result<Self, DictError> {
        if answers.is_empty() {
            return Err(DictError::MissingAnswers);
        }
        let size = answers.get(0).unwrap().len();
        let dict = Dict {
            answers,
            allowed,
            size,
        };
        match dict.check() {
            None => Ok(dict),
            Some(e) => Err(e),
        }
    }
}

// Implement iterators for Dict
impl<'a> IntoParallelRefIterator<'a> for Dict {
    type Item = &'a String;
    type Iter = rayon::iter::Chain<
        rayon::slice::Iter<'a, std::string::String>,
        rayon::slice::Iter<'a, std::string::String>,
    >;

    fn par_iter(&'a self) -> Self::Iter {
        self.answers.par_iter().chain(self.allowed.par_iter())
    }
}
impl std::iter::IntoIterator for Dict {
    type Item = String;
    type IntoIter = std::iter::Chain<std::vec::IntoIter<String>, std::vec::IntoIter<String>>;

    fn into_iter(self) -> Self::IntoIter {
        self.answers.into_iter().chain(self.allowed.into_iter())
    }
}

impl Dict {
    pub fn len(&self) -> usize {
        self.answers.len() + self.allowed.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn word_size(&self) -> usize {
        self.size
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let dict: Dict = serde_json::from_reader(reader)?;

        match dict.check() {
            Some(err) => Err(Box::new(err)),
            None => Ok(dict),
        }
    }
}

impl Dict {
    /// Check the dictionnary consistency
    ///
    /// Performs following checks:
    /// * All words have the same size
    /// * No duplicate exist in the word list
    /// * Only allowed characters (ASCII uppercase) are used
    fn check(&self) -> Option<DictError> {
        self.check_size()
            .or_else(|| self.check_duplicate().or_else(|| self.check_characters()))
    }

    /// Check that all words share the same size
    fn check_size(&self) -> Option<DictError> {
        let size = self.size;
        let invalid_word = self
            .par_iter()
            .find_any(|&word| word.chars().count() != size);

        invalid_word
            .map(|word| DictError::InconsistentSize(size, word.chars().count(), word.to_owned()))
    }

    /// Check list contains no duplicate
    fn check_duplicate(&self) -> Option<DictError> {
        let count_map = self
            .par_iter()
            .fold(HashMap::new, |mut acc, word| {
                let count = acc.entry(word).or_insert(0);
                *count += 1;
                acc
            })
            .reduce(HashMap::new, |m1, m2| {
                m2.iter().fold(m1, |mut acc, (&word, &count_m2)| {
                    let count = acc.entry(word).or_insert(0);
                    *count += count_m2;
                    acc
                })
            });

        let non_unique_word = count_map.par_iter().find_any(|(_, &count)| count != 1);

        non_unique_word.map(|(&word, &count)| DictError::DuplicateWord(count, word.to_owned()))
    }

    /// Check all characters are allowed
    fn check_characters(&self) -> Option<DictError> {
        let invalid_word = self
            .par_iter()
            .find_any(|&word| !word.chars().all(|character| character.is_ascii_uppercase()));

        invalid_word.map(|word| {
            let bad_char = word
                .chars()
                .find(|character| !character.is_ascii_uppercase())
                .unwrap();
            DictError::UnauthorizedCharacter(bad_char, word.to_owned())
        })
    }
}
