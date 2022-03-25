macro_rules! vecstr {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn main() {
    let dict_valid = mutsolver::Dict {
        words: vecstr!["ABACAS", "ABADER", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
    };
    let res = dict_valid.check();
    assert!(res.is_none());

    let dict_invalid_size = mutsolver::Dict {
        words: vecstr!["ABACAS", "ABADE", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
    };
    let res = dict_invalid_size.check();
    println!("Result: {:?}", res);
    assert!(res.is_some());
    println!("Result: {}", res.unwrap());

    let dict_invalid_dupl = mutsolver::Dict {
        words: vecstr!["ABACAS", "ABATEE", "ABAQUE", "ABASIE", "ABATEE", "ABATIS"],
    };
    let res = dict_invalid_dupl.check();
    println!("Result: {:?}", res);
    assert!(res.is_some());
    println!("Result: {}", res.unwrap());

    let dict_invalid_char = mutsolver::Dict {
        words: vecstr!["ABACAS", "ABADER", "ABAQUé", "ABASIE", "ABATEE", "ABATIS"],
    };
    let res = dict_invalid_char.check();
    println!("Result: {:?}", res);
    assert!(res.is_some());
    println!("Result: {}", res.unwrap());

    let dict_read = mutsolver::Dict::from_file("./data/dict/sutom-6A.json").unwrap();
    println!("Dict read: {:?}", dict_read);

    let word = "TOTALEMENT".to_string();
    let options = mutsolver::Options::default();

    let tests = mutsolver::Test::for_word(&word, &options);
    println!("Tests for word '{}': {:?}", word, tests);

    let tests_dict = mutsolver::Test::for_dict(&dict_read, &options);
    println!(
        "Tests for dict SUTOM A6: {:?} (TOTAL COUNT: {})",
        tests_dict,
        tests_dict.len()
    );
}
