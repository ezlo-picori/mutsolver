use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{Dict, Options};
use std::collections::HashMap;

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum Test {
    At(char, usize),      // letter, position
    HasAtLeast(char, u8), // letter, count
    HasAtMost(char, u8),  // letter, count
    HasPrefix(String),    // prefix
    HasSuffix(String),    // suffix
}

impl Clone for Test {
    fn clone(&self) -> Self {
        match self {
            Self::At(l, p) => Self::At(*l, *p),
            Self::HasAtMost(l, c) => Self::HasAtMost(*l, *c),
            Self::HasAtLeast(l, c) => Self::HasAtLeast(*l, *c),
            Self::HasPrefix(prefix) => Self::HasPrefix(prefix.clone()),
            Self::HasSuffix(prefix) => Self::HasSuffix(prefix.clone()),
        }
    }
}

pub type TestSuite = Vec<Test>;

impl Test {
    pub fn for_dict(dict: &Dict, options: &Options) -> TestSuite {
        let test_counts = dict
            .par_iter()
            .fold(HashMap::new, |mut acc, word| {
                Test::for_word(word, options).into_iter().for_each(|test| {
                    let count = acc.entry(test).or_insert(0);
                    *count += 1;
                });
                acc
            })
            .reduce(HashMap::new, |m1, m2| {
                m2.into_iter().fold(m1, |mut acc, (test, count_other)| {
                    let count = acc.entry(test).or_insert(0);
                    *count += count_other;
                    acc
                })
            });

        let word_count = dict.len();
        let min_count = std::cmp::max((word_count as f32 * options.tests_keep_ratio) as usize, 2);
        let max_count = word_count - min_count;

        let mut list_test = TestSuite::new();
        for (test, count) in test_counts.into_iter() {
            if min_count < count && count < max_count {
                list_test.push(test);
            }
        }
        list_test
    }

    /// Generate all tests suited for a given word
    pub fn for_word(word: &str, options: &Options) -> TestSuite {
        let mut list_test = TestSuite::new();

        // Add "At" tests
        word.chars()
            .enumerate()
            .for_each(|(index, letter)| list_test.push(Test::At(letter, index)));

        // Add "HasAtLeast"/"HasAtMost" tests
        let mut letter_count = HashMap::new();

        word.chars().for_each(|letter| {
            let count = letter_count.entry(letter).or_insert(0);
            *count += 1;
        });

        let letter_count = letter_count; // Shadow with immutable
        letter_count.iter().for_each(|(&letter, &count)| {
            list_test.push(Test::HasAtMost(letter, count));
            for c in 1..(count + 1) {
                list_test.push(Test::HasAtLeast(letter, c));
            }
        });

        // Add "HasPrefix"/"HasSuffix" tests
        let len = word.len();
        for p in 2..options.tests_xfix_lengths + 1 {
            list_test.push(Test::HasPrefix(word[..p].to_string()));
            list_test.push(Test::HasSuffix(word[len - p..].to_string()));
        }

        // Return list of tests
        list_test
    }

    // Validate a word against a given test
    pub fn run(&self, word: &str) -> bool {
        match self {
            Self::At(letter, position) => {
                word.chars().nth(*position).map_or(false, |l| l == *letter)
            }
            Self::HasAtLeast(letter, count) => {
                word.chars().filter(|&l| l == *letter).count() >= *count as usize
            }
            Self::HasAtMost(letter, count) => {
                word.chars().filter(|&l| l == *letter).count() <= *count as usize
            }
            Self::HasPrefix(prefix) => word.starts_with(prefix),
            Self::HasSuffix(suffix) => word.ends_with(suffix),
        }
    }
}
