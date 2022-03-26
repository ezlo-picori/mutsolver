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
    /// Dict are simple lists of words
    pub words: WordList,
}

impl Dict {
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
    pub fn check(&self) -> Option<DictError> {
        self.check_size()
            .or_else(|| self.check_duplicate().or_else(|| self.check_characters()))
    }

    /// Check that all words share the same size
    pub fn check_size(&self) -> Option<DictError> {
        let size = self.words[0].chars().count();
        let invalid_word = self
            .words
            .par_iter()
            .find_any(|&word| word.chars().count() != size);

        invalid_word
            .map(|word| DictError::InconsistentSize(size, word.chars().count(), word.to_owned()))
    }

    /// Check list contains no duplicate
    pub fn check_duplicate(&self) -> Option<DictError> {
        let count_map = self
            .words
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
    pub fn check_characters(&self) -> Option<DictError> {
        let invalid_word = self
            .words
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
