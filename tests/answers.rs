use mutsolver_core::{
    Answer::{NO, YES},
    Answers,
};
mod fixtures;
use fixtures::{fixture_dict, fixture_testsuite};

#[test]
fn test_answers_word() {
    let tests = fixture_testsuite();
    let answers = Answers::of_word("ABOUTI", &tests);
    assert_eq!(answers.0, vec![YES, YES, YES, NO]);
}

#[test]
fn test_answers_dict() {
    let tests = fixture_testsuite();
    let dict = fixture_dict();
    let dict_answers = Answers::of_dict(&dict, &tests);
    assert_eq!(
        dict_answers,
        vec![
            Answers(vec![YES, YES, YES, NO]),
            Answers(vec![YES, YES, NO, YES]),
            Answers(vec![NO, NO, YES, NO]),
        ]
    );
}
