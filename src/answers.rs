use crate::{Dict, Test, TestSuite};
/// Structure containing answers for a test suite
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, PartialEq)]
pub struct Answers(Vec<bool>);

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
        let mut dict_answers = DictAnswers::with_capacity(dict.words.len());

        dict.words
            .par_iter()
            .map(|word| Answers::of_word(word, &tests))
            .collect_into_vec(&mut dict_answers);

        dict_answers
    }
}

fn fixture_dict() -> Dict {
    Dict {
        words: vec![
            "ABOUTI".to_string(),
            "ABONDE".to_string(),
            "ASORTI".to_string(),
        ],
    }
}

#[test]
fn test_fixture_dict() {
    let dict = fixture_dict();
    assert!(dict.check().is_none());
}

fn fixture_testsuite() -> TestSuite {
    vec![
        Test::At('B', 1),
        Test::HasPrefix("AB".to_string()),
        Test::HasSuffix("TI".to_string()),
        Test::At('D', 4),
    ]
}

#[test]
fn test_answers_word() {
    let tests = fixture_testsuite();
    let answers = Answers::of_word("ABOUTI", &tests);
    assert_eq!(answers.0, vec![true, true, true, false]);
}

#[test]
fn test_answers_dict() {
    let tests = fixture_testsuite();
    let dict = fixture_dict();
    let dict_answers = Answers::of_dict(&dict, &tests);
    assert_eq!(
        dict_answers,
        vec![
            Answers(vec![true, true, true, false]),
            Answers(vec![true, true, false, true]),
            Answers(vec![false, false, true, false]),
        ]
    );
}
