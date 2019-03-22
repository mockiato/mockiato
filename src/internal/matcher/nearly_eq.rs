use crate::internal::fmt::MaybeDebug;
use crate::internal::{ArgumentMatcher, MaybeDebugWrapper};
use nameof::name_of_type;
use nearly_eq::NearlyEq;
use std::fmt::{self, Debug};

/// Creates a new `ArgumentMatcher` that matches against values using [`NearlyEq`]
pub fn nearly_eq<T, U>(value: T) -> NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    NearlyEqArgumentMatcher {
        value,
        accuracy: T::eps(),
    }
}

/// Creates a new `ArgumentMatcher` that matches against values using [`NearlyEq`]
pub fn nearly_eq_with_accuracy<T, U>(value: T, accuracy: U) -> NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    NearlyEqArgumentMatcher { value, accuracy }
}

pub struct NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    value: T,
    accuracy: U,
}

impl<T, U> Debug for NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of_type!(Self))
            .field("value", &MaybeDebugWrapper(&self.value))
            .field("accuracy", &MaybeDebugWrapper(&self.accuracy))
            .finish()
    }
}

impl<T, U> ArgumentMatcher<T> for NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    fn matches_argument(&self, input: &T) -> bool {
        NearlyEq::eq(&self.value, input, &self.accuracy)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn float_is_not_nearly_equivalent_to_different_float() {
        let first_value = 3.0;
        let second_value = first_value + 1.0;

        assert!(!nearly_eq(first_value).matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equivalent_to_slightly_different_float() {
        let first_value = 3.0;
        let second_value = first_value + 0.000_000_000_000_1;

        assert!(nearly_eq(first_value).matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equal_to_itself() {
        let first_value = 3.0;
        let second_value = first_value;

        assert!(nearly_eq(first_value).matches_argument(&second_value));
    }

    #[test]
    fn float_is_not_nearly_equal_to_different_float_with_no_accuracy() {
        let first_value = 3.0;
        let second_value = first_value + 1.0;
        let accuracy = 0.0;

        assert!(!nearly_eq_with_accuracy(first_value, accuracy).matches_argument(&second_value));
    }

    #[test]
    fn float_is_not_nearly_equal_to_different_float_with_same_accuracy_as_difference() {
        let first_value = 3.0;
        let second_value = first_value + 0.1;
        let accuracy = 0.1;

        assert!(!nearly_eq_with_accuracy(first_value, accuracy).matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equal_to_different_float_low_accuracy() {
        let first_value = 3.0;
        let second_value = first_value + 0.01;
        let accuracy = 0.1;

        assert!(nearly_eq_with_accuracy(first_value, accuracy).matches_argument(&second_value));
    }

    #[test]
    fn float_is_not_nearly_equal_to_different_float_with_highest_accuracy() {
        let first_value = 3.0;
        let second_value = first_value + 0.1;
        let accuracy = 0.0;

        assert!(!nearly_eq_with_accuracy(first_value, accuracy).matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equal_to_itself_with_highest_accuracy() {
        let first_value = 3.0;
        let second_value = first_value;
        let accuracy = 0.0;

        assert!(nearly_eq_with_accuracy(first_value, accuracy).matches_argument(&second_value));
    }
}
