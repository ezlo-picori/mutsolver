use mutsolver_core::{
    attempt::Attempt,
    Answer::{No, Unknown, Yes},
    Answers, Game, Options,
};
mod fixtures;
use fixtures::{fixture_dict, fixture_testsuite};

#[test]
fn test_valid_game() {
    let dict = fixture_dict();
    let mut game = Game::new(&dict, &Options::default());
    assert_eq!(game.attempts().len(), 0);

    let answer = "ASORTI";

    game.add(Attempt::from_answer("ABOUTI", answer).unwrap());
    assert_eq!(game.attempts().len(), 1);
}

#[test]
fn test_known_answers() {
    let dict = fixture_dict();
    let tests = fixture_testsuite();

    let mut game = Game::new_with_tests(&dict, tests);

    assert_eq!(
        game.known_answers().unwrap(),
        Answers(vec![Unknown, Unknown, Unknown, Unknown, Unknown])
    );

    game.add(Attempt::from_answer("ABOUTI", "ABONDE").unwrap());

    assert_eq!(
        game.known_answers().unwrap(),
        Answers(vec![Yes, Yes, No, Unknown, Unknown])
    );

    game.add(Attempt::from_answer("ABSOLU", "ABONDE").unwrap());

    assert_eq!(
        game.known_answers().unwrap(),
        Answers(vec![Yes, Yes, No, Unknown, No])
    );

    game.add(Attempt::from_answer("ABONDE", "ABONDE").unwrap());

    assert_eq!(
        game.known_answers().unwrap(),
        Answers(vec![Yes, Yes, No, Yes, No])
    );
}
