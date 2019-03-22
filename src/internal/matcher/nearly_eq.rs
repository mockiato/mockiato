use crate::internal::fmt::MaybeDebug;
use crate::internal::{ArgumentMatcher, MaybeDebugWrapper};
use nearly_eq::NearlyEq;
use std::fmt;
use std::fmt::Debug;

/// Creates a new `ArgumentMatcher` that matches against values using [`NearlyEq`]
pub fn nearly_eq<T>(value: T) -> NearlyEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    NearlyEqArgumentMatcher { value }
}

pub struct NearlyEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    value: T,
}

impl<T> Debug for NearlyEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&MaybeDebugWrapper(&self.value), f)
    }
}

impl<T, U> ArgumentMatcher<U> for NearlyEqArgumentMatcher<T>
where
    T: NearlyEq<U> + MaybeDebug,
{
    fn matches_argument(&self, input: &U) -> bool {
        NearlyEq::eq(&self.value, input, &T::eps())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nearly_eq_works_with_different_f64() {
        let first_value = 3.0;
        let second_value = first_value + 1.0;

        assert!(!nearly_eq(first_value).matches_argument(&second_value));
    }

    #[test]
    fn nearly_eq_works_with_slightly_different_f64() {
        let first_value = 3.0;
        let second_value = first_value + 0.000_000_000_000_1;

        assert!(nearly_eq(first_value).matches_argument(&second_value));
    }

    #[test]
    fn nearly_eq_works_with_same_f64() {
        let first_value = 3.0;
        let second_value = first_value;

        assert!(nearly_eq(first_value).matches_argument(&second_value));
    }
}
