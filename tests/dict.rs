use mutsolver_core::{Dict, DictError};

macro_rules! vecstr {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn test_dict_valid() {
    let dict = Dict {
        words: vecstr!["ABACAS", "ABADER", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
    };
    assert!(dict.check().is_none());
}

// When feature(assert_matches) is stabilized, use it instead of the match statement
// in following tests.

#[test]
fn test_dict_invalid_size() {
    let dict = Dict {
        words: vecstr!["ABACAS", "ABADE", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
    };
    match dict.check() {
        Some(DictError::InconsistentSize(6, 5, _)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_dict_invalid_duplicate() {
    let dict = Dict {
        words: vecstr!["ABACAS", "ABATEE", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
    };
    match dict.check() {
        Some(DictError::DuplicateWord(2, _)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_dict_invalid_character() {
    let dict = Dict {
        words: vecstr!["ABACAS", "ABADER", "ABAQUé", "ABASIE", "ABATEE", "ABATIS"],
    };
    match dict.check() {
        Some(DictError::UnauthorizedCharacter('é', _)) => (),
        _ => panic!(),
    }
}
