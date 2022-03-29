use mutsolver_core::{Dict, Test, TestSuite};

#[allow(dead_code)]
pub fn fixture_dict() -> Dict {
    Dict::new(
        vec![
            "ABOUTI".to_string(),
            "ABONDE".to_string(),
            "ASORTI".to_string(),
            "ABSOLU".to_string(),
        ],
        vec![],
    )
    .unwrap()
}

#[allow(dead_code)]
pub fn fixture_testsuite() -> TestSuite {
    vec![
        Test::At('B', 1),
        Test::HasPrefix("AB".to_string()),
        Test::HasSuffix("TI".to_string()),
        Test::At('D', 4),
        Test::HasAtLeast('L', 1),
    ]
}
