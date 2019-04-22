use super::ArgumentMatcher;
use std::fmt::Write;
use std::fmt::{self, Display};

/// Creates a new `ArgumentMatcher` that matches any value.
pub fn any() -> AnyArgumentMatcher {
    AnyArgumentMatcher
}

#[derive(Debug)]
pub struct AnyArgumentMatcher;

impl Display for AnyArgumentMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('*')
    }
}

impl<'args, U> ArgumentMatcher<U> for AnyArgumentMatcher {
    fn matches_argument(&self, _input: &U) -> bool {
        true
    }
}
