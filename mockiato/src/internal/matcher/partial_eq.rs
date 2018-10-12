use super::ArgumentMatcher;
use crate::internal::debug::MaybeDebug;
use std::fmt::{self, Debug};

pub(crate) struct PartialEqMatcher<T>(T)
where
    T: PartialEq + MaybeDebug;

impl<T> From<T> for PartialEqMatcher<T>
where
    T: PartialEq + MaybeDebug,
{
    fn from(value: T) -> Self {
        PartialEqMatcher(value)
    }
}

impl<T> Debug for PartialEqMatcher<T>
where
    T: PartialEq + MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<T> ArgumentMatcher<T> for PartialEqMatcher<T>
where
    T: PartialEq + MaybeDebug,
{
    fn matches_argument(&self, input: &T) -> bool {
        self.0 == *input
    }
}
