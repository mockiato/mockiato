use crate::internal::arguments::Arguments;
use crate::internal::debug::MaybeDebug;
use std::fmt::Debug;

mod partial_eq;

pub trait ArgumentMatcher<T>: MaybeDebug {
    fn matches_argument(&self, input: &T) -> bool;
}

pub trait ArgumentsMatcher<'args>: Debug {
    type Arguments: Arguments;

    fn matches_arguments(&self, input: &Self::Arguments) -> bool;
}

impl<T, U> ArgumentMatcher<U> for T
where
    T: PartialEq<U> + MaybeDebug + 'static,
{
    fn matches_argument(&self, input: &U) -> bool {
        self == input
    }
}
