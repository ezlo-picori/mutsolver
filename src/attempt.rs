use crate::answers::Answer;
use crate::errors::Error;
use crate::state::{State, States};
use crate::tests::Test;

/// An attempt represent as word tested in a game and the state of each
/// character of this word.
pub struct Attempt<'a>(pub &'a str, pub States);

/// Attempts contains all the attempts of a given game
pub type Attempts<'a> = Vec<Attempt<'a>>;

impl<'a, 'b> Attempt<'a> {
    pub fn from_answer(attempt: &'a str, answer: &'b str) -> Result<Self, Error> {
        use std::collections::HashMap;

        let size = answer.chars().count();
        if attempt.chars().count() != size {
            return Err(Error::InvalidSize(
                size,
                attempt.chars().count(),
                attempt.to_owned(),
            ));
        }
        let mut states = vec![State::No; size];

        answer
            .chars()
            .zip(attempt.chars())
            .zip(states.iter_mut())
            .filter(|((ans, att), _)| *ans == *att)
            .for_each(|(_, sta)| *sta = State::Yes);

        // Count not-identified characters
        let mut acc = answer
            .chars()
            .zip(states.iter())
            .filter(|(_, sta)| **sta != State::Yes)
            .fold(HashMap::new(), |mut acc, (c, _)| {
                let count = acc.entry(c).or_insert(0);
                *count += 1;
                acc
            });

        // Traverse attempted incorrect characters and mark them MEH if
        // answer counter for this character is non-null (then decrement
        // said counter)
        for (sta, c) in states
            .iter_mut()
            .zip(attempt.chars())
            .filter(|(sta, _)| **sta != State::Yes)
        {
            if let Some(count) = acc.get_mut(&c) {
                *sta = State::Meh;
                *count -= 1;
            }
        }

        Ok(Attempt(attempt, states))
    }

    pub fn answers(&self, test: &Test) -> Result<Answer, Error> {
        let states = &self.1;
        match test {
            Test::At(tl, tp) => {
                // test letter, test position
                let letter = self
                    .0
                    .chars()
                    .nth(*tp)
                    .ok_or_else(|| Error::UnexpectedTest(test.clone(), self.0.to_owned()))?;
                let state = states.get(*tp).unwrap();
                match state {
                    State::Yes => {
                        if *tl == letter {
                            Ok(Answer::Yes)
                        } else {
                            Ok(Answer::No)
                        }
                    }
                    State::Meh | State::No => {
                        if *tl == letter {
                            Ok(Answer::No)
                        } else {
                            Ok(Answer::Unknown)
                        }
                    }
                }
            }
            Test::HasAtMost(tl, tc) => {
                // test letter, test count
                let occurences = self.0.chars().filter(|l| *l == *tl).count();
                let valid_occurences = self
                    .0
                    .chars()
                    .zip(states.iter())
                    .filter(|(l, s)| *l == *tl && **s != State::No)
                    .count();

                if occurences == valid_occurences {
                    Ok(Answer::Unknown)
                } else if valid_occurences <= *tc as usize {
                    Ok(Answer::Yes)
                } else {
                    Ok(Answer::No)
                }
            }
            Test::HasAtLeast(tl, tc) => {
                // test letter, test count
                let letter_count = self.0.chars().filter(|l| *l == *tl).count();
                if letter_count < *tc as usize {
                    Ok(Answer::Unknown)
                } else if self
                    .0
                    .chars()
                    .zip(states.iter())
                    .filter(|(l, s)| *l == *tl && **s != State::No)
                    .count()
                    >= *tc as usize
                {
                    Ok(Answer::Yes)
                } else {
                    Ok(Answer::No)
                }
            }
            Test::HasPrefix(prefix) => {
                if prefix.len() > self.0.chars().count() {
                    Err(Error::UnexpectedTest(test.clone(), self.0.to_owned()))
                } else if prefix
                    .chars()
                    .zip(self.0.chars())
                    .zip(self.1.iter())
                    .all(|((pc, ac), st)| pc == ac && *st == State::Yes)
                {
                    Ok(Answer::Yes)
                } else if prefix.chars().zip(self.0.chars()).zip(self.1.iter()).any(
                    |((pc, ac), st)| pc != ac && *st == State::Yes || pc == ac && *st != State::Yes,
                ) {
                    Ok(Answer::No)
                } else {
                    Ok(Answer::Unknown)
                }
            }
            Test::HasSuffix(suffix) => {
                if suffix.len() > self.0.chars().count() {
                    return Err(Error::UnexpectedTest(test.clone(), self.0.to_owned()));
                }

                let start = self.0.chars().count() - suffix.len();

                if suffix
                    .chars()
                    .zip(self.0.chars().skip(start))
                    .zip(self.1.iter().skip(start))
                    .all(|((sc, ac), st)| sc == ac && *st == State::Yes)
                {
                    Ok(Answer::Yes)
                } else if suffix
                    .chars()
                    .zip(self.0.chars().skip(start))
                    .zip(self.1.iter().skip(start))
                    .any(|((sc, ac), st)| {
                        sc != ac && *st == State::Yes || sc == ac && *st != State::Yes
                    })
                {
                    Ok(Answer::No)
                } else {
                    Ok(Answer::Unknown)
                }
            }
        }
    }
}
