pub struct Options {
    // Max length of prefixes/suffixes searched for in tests
    pub tests_xfix_lengths: usize,
    // Minimum percentage of words matching a test for it to be kept
    pub tests_keep_ratio: f32,
}

impl Options {
    pub fn default() -> Self {
        Options {
            tests_xfix_lengths: 4,
            tests_keep_ratio: 0.01,
        }
    }
}
