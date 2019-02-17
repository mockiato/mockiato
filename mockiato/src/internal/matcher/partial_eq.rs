use super::ArgumentMatcher;
use crate::internal::fmt::{MaybeDebug, MaybeDebugWrapper};
use std::fmt::{self, Debug};

/// Creates a new `ArgumentMatcher` that matches against values using [`PartialEq`]
pub fn partial_eq<T>(value: T) -> PartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    PartialEqArgumentMatcher { value }
}

/// Creates a new `ArgumentMatcher` that matches against values using [`PartialEq`].
/// Supports comparing a reference against an owned value.
pub fn partial_eq_owned<T>(value: T) -> OwnedPartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    OwnedPartialEqArgumentMatcher { value }
}

pub struct PartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    value: T,
}

impl<T> Debug for PartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&MaybeDebugWrapper(&self.value), f)
    }
}

impl<T, U> ArgumentMatcher<U> for PartialEqArgumentMatcher<T>
where
    T: PartialEq<U> + MaybeDebug,
{
    fn matches_argument(&self, input: &U) -> bool {
        &self.value == input
    }
}

pub struct OwnedPartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    value: T,
}

impl<T> Debug for OwnedPartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&MaybeDebugWrapper(&self.value), f)
    }
}

impl<'args, T, U> ArgumentMatcher<&'args U> for OwnedPartialEqArgumentMatcher<T>
where
    T: PartialEq<U> + MaybeDebug,
{
    fn matches_argument(&self, input: &&U) -> bool {
        &self.value == *input
    }
}
