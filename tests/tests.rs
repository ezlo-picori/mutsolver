use mutsolver_core::{Options, Test};

#[test]
fn run_at_test() {
    // "At" test should succeed if word contains the letter at given position
    assert!(Test::At('A', 0).run("ABCDE"));
    assert!(Test::At('D', 3).run("ABCDE"));

    // "At" test should fail if word does not have the letter at given position
    assert!(!Test::At('A', 1).run("ABCDE"));
    assert!(!Test::At('Z', 0).run("ABCDE"));

    // "At" test should fail if given position is out of range
    assert!(!Test::At('A', 9).run("ABCDE"));
}

#[test]
fn run_has_at_least_test() {
    assert!(Test::HasAtLeast('A', 1).run("ABCDE"));
    assert!(!Test::HasAtLeast('A', 2).run("ABCDE"));
    assert!(Test::HasAtLeast('A', 2).run("ABCADE"));
    assert!(Test::HasAtLeast('Z', 0).run("ABCDE"));
    assert!(!Test::HasAtLeast('Z', 1).run("ABCDE"));
}

#[test]
fn run_has_at_most_test() {
    assert!(!Test::HasAtMost('A', 0).run("ABCDE"));
    assert!(Test::HasAtMost('A', 1).run("ABCDE"));
    assert!(Test::HasAtMost('A', 2).run("ABCDE"));
    assert!(Test::HasAtMost('A', 2).run("ABCADE"));
    assert!(!Test::HasAtMost('A', 2).run("ABCADEA"));
    assert!(Test::HasAtMost('Z', 1).run("ABCDE"));
}

#[test]
fn run_has_prefix_test() {
    assert!(Test::HasPrefix("A".to_string()).run("ABCDE"));
    assert!(Test::HasPrefix("AB".to_string()).run("ABCDE"));
    assert!(Test::HasPrefix("ABC".to_string()).run("ABCDE"));
    assert!(Test::HasPrefix("ABCD".to_string()).run("ABCDE"));
    assert!(Test::HasPrefix("ABCDE".to_string()).run("ABCDE"));
    assert!(!Test::HasPrefix("ABCDEF".to_string()).run("ABCDE"));
    assert!(!Test::HasPrefix("ZAB".to_string()).run("ABCDE"));
}

#[test]
fn run_has_suffix_test() {
    assert!(Test::HasSuffix("E".to_string()).run("ABCDE"));
    assert!(Test::HasSuffix("DE".to_string()).run("ABCDE"));
    assert!(Test::HasSuffix("CDE".to_string()).run("ABCDE"));
    assert!(Test::HasSuffix("BCDE".to_string()).run("ABCDE"));
    assert!(Test::HasSuffix("ABCDE".to_string()).run("ABCDE"));
    assert!(!Test::HasSuffix("ZABCDE".to_string()).run("ABCDE"));
    assert!(!Test::HasSuffix("DEZ".to_string()).run("ABCDE"));
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
