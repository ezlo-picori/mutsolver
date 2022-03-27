use mutsolver_core::{game::GameError, Answer, Attempt, Game, State, Test};
use State::{MEH, NO, YES};
mod fixtures;
use fixtures::fixture_dict;

#[test]
fn test_new_attempt_only_yes() {
    let answer = "ABCD";
    let attempt = Attempt::from_answer("AZCZ", answer).unwrap();
    assert_eq!(
        attempt.1,
        vec![State::YES, State::NO, State::YES, State::NO]
    )
}

#[test]
fn test_new_attempt_mixed_yes_meh() {
    let answer = "REVOLVER";
    let attempt = Attempt::from_answer("RIVIERES", answer).unwrap();
    assert_eq!(attempt.1, vec![YES, NO, YES, NO, MEH, MEH, YES, NO])
}

#[test]
fn test_new_attempt_invalid_sizes() {
    let answer = "ABCD";
    let attempt = Attempt::from_answer("ADCZE", answer);
    match attempt {
        Err(GameError::InvalidSize(4, 5, _)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_valid_game() {
    let dict = fixture_dict();
    let mut game = Game::new(&dict);
    assert_eq!(game.attempts().len(), 0);

    let answer = "ASORTI";

    game.add(Attempt::from_answer("ABOUTI", answer).unwrap());
    assert_eq!(game.attempts().len(), 1);
}

#[test]
fn test_attempt_answer_at() {
    let attempt = Attempt("ABCD", vec![YES, MEH, NO, MEH]);
    match attempt.answers(&Test::At('A', 0)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::At('B', 1)) {
        Ok(Answer::NO) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::At('Z', 2)) {
        Ok(Answer::UNKNOWN) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::At('D', 10)) {
        Err(GameError::UnexpectedTest(_, _)) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hasatleast() {
    let attempt = Attempt("ABBCCC", vec![YES, YES, MEH, NO, YES, MEH]);
    match attempt.answers(&Test::HasAtLeast('A', 1)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('A', 2)) {
        Ok(Answer::UNKNOWN) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('B', 2)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 1)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 2)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 3)) {
        Ok(Answer::NO) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtLeast('C', 4)) {
        Ok(Answer::UNKNOWN) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hasatmost() {
    let attempt = Attempt("ABBCCCD", vec![YES, YES, NO, NO, YES, MEH, NO]);
    match attempt.answers(&Test::HasAtMost('A', 1)) {
        Ok(Answer::UNKNOWN) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('B', 1)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('B', 2)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 1)) {
        Ok(Answer::NO) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 2)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 3)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('C', 4)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('D', 1)) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasAtMost('Z', 2)) {
        Ok(Answer::UNKNOWN) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hasprefix() {
    let attempt = Attempt("ABCDE", vec![YES, YES, YES, YES, YES]);
    match attempt.answers(&Test::HasPrefix("ABC".to_string())) {
        Ok(Answer::YES) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasPrefix("AZC".to_string())) {
        Ok(Answer::NO) => (),
        _ => panic!(),
    };

    let attempt = Attempt("ABCDE", vec![YES, MEH, YES, YES, YES]);
    match attempt.answers(&Test::HasPrefix("AZC".to_string())) {
        Ok(Answer::UNKNOWN) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasPrefix("ABC".to_string())) {
        Ok(Answer::NO) => (),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasPrefix("ABCDEFS".to_string())) {
        Err(GameError::UnexpectedTest(_, _)) => (),
        _ => panic!(),
    };
}

#[test]
fn test_attempt_answer_hassuffix() {
    let attempt = Attempt("ABCDE", vec![YES, YES, YES, YES, YES]);
    match attempt.answers(&Test::HasSuffix("CDE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::YES),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("ZDE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::NO),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("CZE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::NO),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("CDZ".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::NO),
        _ => panic!(),
    };

    let attempt = Attempt("ABCDE", vec![YES, YES, YES, MEH, YES]);
    match attempt.answers(&Test::HasSuffix("CZE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::UNKNOWN),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("CDE".to_string())) {
        Ok(answer) => assert_eq!(answer, Answer::NO),
        _ => panic!(),
    };
    match attempt.answers(&Test::HasSuffix("ABCDEFS".to_string())) {
        Err(GameError::UnexpectedTest(_, _)) => (),
        _ => panic!(),
    };
}
