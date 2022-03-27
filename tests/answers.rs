use mutsolver_core::Answers;
mod fixtures;
use fixtures::{fixture_dict, fixture_testsuite};

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
