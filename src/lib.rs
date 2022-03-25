use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug)]
pub enum DictError<'a> {
    InconsistentSize(usize, usize, &'a str), // expected size, found size, incriminated word
    DuplicateWord(usize, &'a str),           // incriminated word count and value
    UnauthorizedCharacter(char, &'a str),    // incriminated character and word
}

impl<'a> Error for DictError<'a> {}

impl<'a> std::fmt::Display for DictError<'a> {
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Dict<'a> {
    /// Dict are simple lists of words
    #[serde(borrow)]
    pub words: Vec<&'a str>,
}

impl<'a> Dict<'a> {
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
            .find_any(|&&word| word.chars().count() != size);

        invalid_word.map(|&word| DictError::InconsistentSize(size, word.chars().count(), word))
    }

    /// Check list contains no duplicate
    pub fn check_duplicate(&self) -> Option<DictError> {
        let count_map = self
            .words
            .par_iter()
            .fold(HashMap::new, |mut acc, &word| {
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

        non_unique_word.map(|(&word, &count)| DictError::DuplicateWord(count, word))
    }

    /// Check all characters are allowed
    pub fn check_characters(&self) -> Option<DictError> {
        let invalid_word = self
            .words
            .par_iter()
            .find_any(|&&word| !word.chars().all(|character| character.is_ascii_uppercase()));

        invalid_word.map(|&word| {
            let bad_char = word
                .chars()
                .find(|character| !character.is_ascii_uppercase())
                .unwrap();
            DictError::UnauthorizedCharacter(bad_char, word)
        })
    }
}
