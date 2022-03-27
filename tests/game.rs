use mutsolver_core::{game::GameError, Attempt, Game, State};
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
    use State::{MEH, NO, YES};
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
