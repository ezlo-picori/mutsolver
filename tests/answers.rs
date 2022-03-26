use mutsolver_core::{Answers, Dict, Test, TestSuite};

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
