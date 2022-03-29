/// A State represents the answer given for a character in an attempt.
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    /// Letter is correctly placed (RED).
    Yes,
    // letter is incorrectly placed (YELLOW)
    Meh,
    /// letter is not in answer (BLUE)
    No,
}

/// A States contains the State of each individual character of an attempt.
pub type States = Vec<State>;
