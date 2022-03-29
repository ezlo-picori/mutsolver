use mutsolver_core::{
    Answer::{No, Yes},
    Answers,
};
mod fixtures;
use fixtures::{fixture_dict, fixture_testsuite};

#[test]
fn test_answers_word() {
    let tests = fixture_testsuite();
    let answers = Answers::of_word("ABOUTI", &tests);
    assert_eq!(answers.0, vec![Yes, Yes, Yes, No]);
}

#[test]
fn test_answers_dict() {
    let tests = fixture_testsuite();
    let dict = fixture_dict();
    let dict_answers = Answers::of_dict(&dict, &tests);
    assert_eq!(
        dict_answers,
        vec![
            Answers(vec![Yes, Yes, Yes, No]),
            Answers(vec![Yes, Yes, No, Yes]),
            Answers(vec![No, No, Yes, No]),
        ]
    );
}
