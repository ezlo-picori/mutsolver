use mutsolver_core::{attempt::Attempt, errors::Error, state::State, Answer, Test};
use State::{Meh, No, Yes};

#[test]
fn test_new_attempt_only_yes() {
    let answer = "ABCD";
    let attempt = Attempt::from_answer("AZCZ", answer).unwrap();
    assert_eq!(attempt.1, vec![Yes, No, Yes, No])
}

#[test]
fn test_new_attempt_mixed_yes_meh() {
    let answer = "REVOLVER";
    let attempt = Attempt::from_answer("RIVIERES", answer).unwrap();
    assert_eq!(attempt.1, vec![Yes, No, Yes, No, Meh, Meh, Yes, No])
}

#[test]
fn test_new_attempt_invalid_sizes() {
    let answer = "ABCD";
    let attempt = Attempt::from_answer("ADCZE", answer);
    match attempt {
        Err(Error::InvalidSize(4, 5, _)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_attempt_answer_at() {
    let attempt = Attempt("ABCD", vec![Yes, Meh, No, Meh]);
    match attempt.answers(&Test::At('A', 0)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::At('B', 1)) {
        Ok(Answer::No) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::At('Z', 2)) {
        Ok(Answer::Unknown) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::At('D', 10)) {
        Err(Error::UnexpectedTest(_, _)) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hasatleast() {
    let attempt = Attempt("ABBCCC", vec![Yes, Yes, Meh, No, Yes, Meh]);
    match attempt.answers(&Test::HasAtLeast('A', 1)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('A', 2)) {
        Ok(Answer::Unknown) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('B', 2)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 1)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 2)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 3)) {
        Ok(Answer::No) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 4)) {
        Ok(Answer::Unknown) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hasatmost() {
    let attempt = Attempt("ABBCCCD", vec![Yes, Yes, No, No, Yes, Meh, No]);
    match attempt.answers(&Test::HasAtMost('A', 1)) {
        Ok(Answer::Unknown) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('B', 1)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('B', 2)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 1)) {
        Ok(Answer::No) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 2)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 3)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 4)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('D', 1)) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('Z', 2)) {
        Ok(Answer::Unknown) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hasprefix() {
    let attempt = Attempt("ABCDE", vec![Yes, Yes, Yes, Yes, Yes]);
    match attempt.answers(&Test::HasPrefix("ABC".to_string())) {
        Ok(Answer::Yes) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasPrefix("AZC".to_string())) {
        Ok(Answer::No) => (),
        _ => panic!(),
    };

    let attempt = Attempt("ABCDE", vec![Yes, Meh, Yes, Yes, Yes]);
    match attempt.answers(&Test::HasPrefix("AZC".to_string())) {
        Ok(Answer::Unknown) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasPrefix("ABC".to_string())) {
        Ok(Answer::No) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasPrefix("ABCDEFS".to_string())) {
        Err(Error::UnexpectedTest(_, _)) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hassuffix() {
    let attempt = Attempt("ABCDE", vec![Yes, Yes, Yes, Yes, Yes]);
    match attempt.answers(&Test::HasSuffix("CDE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::Yes),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("ZDE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::No),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("CZE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::No),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("CDZ".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::No),
        _ => panic!(),
    };

    let attempt = Attempt("ABCDE", vec![Yes, Yes, Yes, Meh, Yes]);
    match attempt.answers(&Test::HasSuffix("CZE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::Unknown),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("CDE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::No),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("ABCDEFS".to_string())) {
        Err(Error::UnexpectedTest(_, _)) => (),
        _ => panic!(),
    };
}
