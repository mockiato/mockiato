use crate::internal::arguments::Arguments;
use crate::internal::fmt::{MaybeDebug, MaybeDebugWrapper};
use std::fmt::{self, Debug};

/// Creates a new `ArgumentMatcher` that matches against values using [`PartialEq`]
pub fn partial_eq<T>(value: T) -> PartialEqArgumentMatcher<T>
where
    T: MaybeDebug + 'static,
{
    PartialEqArgumentMatcher { value }
}

/// Creates a new `ArgumentMatcher` that matches against values using [`PartialEq`].
/// Supports comparing a reference against an owned value.
pub fn partial_eq_owned<T>(value: T) -> OwnedPartialEqArgumentMatcher<T>
where
    T: MaybeDebug + 'static,
{
    OwnedPartialEqArgumentMatcher { value }
}

pub trait ArgumentMatcher<T>: MaybeDebug {
    fn matches_argument(&self, input: &T) -> bool;
}

pub trait ArgumentsMatcher<'args>: Debug {
    type Arguments: Arguments;

    fn matches_arguments(&self, _input: &Self::Arguments) -> bool {
        // Since argument matchers for methods without any arguments should always match, we can
        // fall back to the default impl on the trait `ArgumentsMatcher`.
        true
    }
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
    T: PartialEq<U> + MaybeDebug + 'static,
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
    T: PartialEq<U> + MaybeDebug + 'static,
{
    fn matches_argument(&self, input: &&U) -> bool {
        &self.value == *input
    }
}

#[cfg(test)]
pub(crate) use self::mock::*;

#[cfg(test)]
mod mock {
    use super::ArgumentsMatcher;
    use crate::internal::arguments::ArgumentsMock;
    use std::cell::RefCell;
    use std::thread::panicking;

    #[derive(Debug, Default)]
    pub(crate) struct ArgumentsMatcherMock {
        matches_arguments_return: Option<bool>,
        matches_arguments_was_called: RefCell<bool>,
    }

    impl ArgumentsMatcherMock {
        pub(crate) fn new(matches_arguments_return: Option<bool>) -> Self {
            Self {
                matches_arguments_return,
                matches_arguments_was_called: RefCell::new(false),
            }
        }
    }

    impl<'args> ArgumentsMatcher<'args> for ArgumentsMatcherMock {
        type Arguments = ArgumentsMock;

        fn matches_arguments(&self, _input: &Self::Arguments) -> bool {
            *self.matches_arguments_was_called.borrow_mut() = true;
            self.matches_arguments_return.unwrap()
        }
    }

    impl Drop for ArgumentsMatcherMock {
        fn drop(&mut self) {
            if !panicking() {
                if self.matches_arguments_return.is_some() {
                    assert!(
                        *self.matches_arguments_was_called.borrow(),
                        "matches_arguments() was never called"
                    );
                }
            }
        }
    }
}
