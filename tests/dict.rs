use mutsolver_core::{Dict, DictError};

macro_rules! vecstr {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn test_dict_valid() {
    let dict = Dict::new(
        vecstr!["ABACAS", "ABADER", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
        vec![],
    );
    assert!(dict.is_ok());
    let dict = dict.unwrap();
    assert_eq!(dict.len(), 6);
    assert!(!dict.is_empty());
}

// When feature(assert_matches) is stabilized, use it instead of the match statement
// in following tests.

#[test]
fn test_dict_invalid_size() {
    let dict = Dict::new(
        vecstr!["ABACAS", "ABADE", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
        vec![],
    );
    match dict {
        Err(DictError::InconsistentSize(6, 5, _)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_dict_missing_answers() {
    let dict = Dict::new(vec![], vecstr!["ABACAS", "ABADE"]);
    match dict {
        Err(DictError::MissingAnswers) => (),
        _ => panic!(),
    }
}

#[test]
fn test_dict_invalid_duplicate() {
    let dict = Dict::new(
        vecstr!["ABACAS", "ABATEE", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
        vec![],
    );
    match dict {
        Err(DictError::DuplicateWord(2, _)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_dict_invalid_duplicate_both() {
    let dict = Dict::new(
        vecstr!["ABACAS", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
        vecstr!["ABATEE"],
    );
    match dict {
        Err(DictError::DuplicateWord(2, _)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_dict_invalid_character() {
    let dict = Dict::new(
        vecstr!["ABACAS", "ABADER", "ABAQUé", "ABASIE", "ABATEE", "ABATIS"],
        vec![],
    );
    match dict {
        Err(DictError::UnauthorizedCharacter('é', _)) => (),
        _ => panic!(),
    }
}
