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

pub type TestSuite = Vec<Test>;

impl Test {
    pub fn for_dict(dict: &Dict, options: &Options) -> TestSuite {
        let test_counts = dict
            .words
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

        let word_count = dict.words.len();
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

#[test]
fn run_at_test() {
    // "At" test should succeed if word contains the letter at given position
    assert!(crate::Test::At('A', 0).run("ABCDE"));
    assert!(crate::Test::At('D', 3).run("ABCDE"));

    // "At" test should fail if word does not have the letter at given position
    assert!(!crate::Test::At('A', 1).run("ABCDE"));
    assert!(!crate::Test::At('Z', 0).run("ABCDE"));

    // "At" test should fail if given position is out of range
    assert!(!crate::Test::At('A', 9).run("ABCDE"));
}

#[test]
fn run_has_at_least_test() {
    assert!(crate::Test::HasAtLeast('A', 1).run("ABCDE"));
    assert!(!crate::Test::HasAtLeast('A', 2).run("ABCDE"));
    assert!(crate::Test::HasAtLeast('A', 2).run("ABCADE"));
    assert!(crate::Test::HasAtLeast('Z', 0).run("ABCDE"));
    assert!(!crate::Test::HasAtLeast('Z', 1).run("ABCDE"));
}

#[test]
fn run_has_at_most_test() {
    assert!(!crate::Test::HasAtMost('A', 0).run("ABCDE"));
    assert!(crate::Test::HasAtMost('A', 1).run("ABCDE"));
    assert!(crate::Test::HasAtMost('A', 2).run("ABCDE"));
    assert!(crate::Test::HasAtMost('A', 2).run("ABCADE"));
    assert!(!crate::Test::HasAtMost('A', 2).run("ABCADEA"));
    assert!(crate::Test::HasAtMost('Z', 1).run("ABCDE"));
}

#[test]
fn run_has_prefix_test() {
    assert!(crate::Test::HasPrefix("A".to_string()).run("ABCDE"));
    assert!(crate::Test::HasPrefix("AB".to_string()).run("ABCDE"));
    assert!(crate::Test::HasPrefix("ABC".to_string()).run("ABCDE"));
    assert!(crate::Test::HasPrefix("ABCD".to_string()).run("ABCDE"));
    assert!(crate::Test::HasPrefix("ABCDE".to_string()).run("ABCDE"));
    assert!(!crate::Test::HasPrefix("ABCDEF".to_string()).run("ABCDE"));
    assert!(!crate::Test::HasPrefix("ZAB".to_string()).run("ABCDE"));
}

#[test]
fn run_has_suffix_test() {
    assert!(crate::Test::HasSuffix("E".to_string()).run("ABCDE"));
    assert!(crate::Test::HasSuffix("DE".to_string()).run("ABCDE"));
    assert!(crate::Test::HasSuffix("CDE".to_string()).run("ABCDE"));
    assert!(crate::Test::HasSuffix("BCDE".to_string()).run("ABCDE"));
    assert!(crate::Test::HasSuffix("ABCDE".to_string()).run("ABCDE"));
    assert!(!crate::Test::HasSuffix("ZABCDE".to_string()).run("ABCDE"));
    assert!(!crate::Test::HasSuffix("DEZ".to_string()).run("ABCDE"));
}

#[test]
fn get_for_word() {
    let options = Options {
        tests_keep_ratio: 0.01,
        tests_xfix_lengths: 2,
    };
    let test_suite = Test::for_word("SEER", &options);

    type Predicate = fn(&Test) -> bool;
    let predicates: [(Predicate, usize); 5] = [
        (|test| matches!(test, Test::At(_, _)), 4),
        (|test| matches!(test, Test::HasPrefix(_)), 1),
        (|test| matches!(test, Test::HasSuffix(_)), 1),
        (|test| matches!(test, Test::HasAtLeast('E', 2)), 1),
        (|test| matches!(test, Test::HasAtLeast(_, 1)), 3),
    ];

    for (predicate, expected_count) in predicates.iter() {
        assert_eq!(
            test_suite.iter().filter(|test| predicate(test)).count(),
            *expected_count
        );
    }
}
