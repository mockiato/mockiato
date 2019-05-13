use crate::internal::argument::Argument;
use crate::internal::fmt::MaybeDebug;
use crate::internal::fmt::MaybeDebugWrapper;
use crate::internal::ArgumentMatcher;
use nameof::name_of;
use nearly_eq::NearlyEq;
use std::fmt::{self, Debug, Display};

impl Argument {
    /// Creates an argument matcher that matches values using [`NearlyEq`].
    /// Uses the default epsilon value defined by [`NearlyEq`].
    ///
    /// # Examples
    /// ```
    /// use mockiato::mockable;
    ///
    /// #[cfg_attr(test, mockable)]
    /// trait FloatFormatter {
    ///     fn format_float(&self, value: f64) -> String;
    /// }
    ///
    /// let expected_result = String::from("0.3");
    /// let mut formatter = FloatFormatterMock::new();
    /// formatter
    ///     .expect_format_float(|a| a.nearly_eq(0.3))
    ///     .returns(expected_result.clone());
    ///
    /// assert_eq!(expected_result, formatter.format_float(0.1 + 0.2),)
    /// ```
    pub fn nearly_eq<T, U>(&self, value: T) -> NearlyEqArgumentMatcher<T, U>
    where
        T: NearlyEq<T, U> + MaybeDebug,
        U: MaybeDebug,
    {
        NearlyEqArgumentMatcher {
            value,
            accuracy: T::eps(),
        }
    }

    /// Creates an argument matcher that matches values using [`NearlyEq`].
    /// A custom epislon value can be passed.
    ///
    /// # Examples
    /// ```
    /// use mockiato::mockable;
    ///
    /// #[cfg_attr(test, mockable)]
    /// trait FloatFormatter {
    ///     fn format_float(&self, value: f64) -> String;
    /// }
    ///
    /// let expected_result = String::from("0.3");
    /// let mut formatter = FloatFormatterMock::new();
    /// formatter
    ///     .expect_format_float(|a| a.nearly_eq_with_accuracy(0.3, 1e-10))
    ///     .returns(expected_result.clone());
    ///
    /// assert_eq!(expected_result, formatter.format_float(0.1 + 0.2),)
    /// ```
    pub fn nearly_eq_with_accuracy<T, U>(
        &self,
        value: T,
        accuracy: U,
    ) -> NearlyEqArgumentMatcher<T, U>
    where
        T: NearlyEq<T, U> + MaybeDebug,
        U: MaybeDebug,
    {
        NearlyEqArgumentMatcher { value, accuracy }
    }
}

pub struct NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    value: T,
    accuracy: U,
}

impl<T, U> Display for NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}±{:?}",
            MaybeDebugWrapper(&self.value),
            MaybeDebugWrapper(&self.accuracy)
        )
    }
}

impl<T, U> Debug for NearlyEqArgumentMatcher<T, U>
where
    T: NearlyEq<T, U> + MaybeDebug,
    U: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of!(type NearlyEqArgumentMatcher<T, U>))
            .field(name_of!(value in Self), &MaybeDebugWrapper(&self.value))
            .field(
                name_of!(accuracy in Self),
                &MaybeDebugWrapper(&self.accuracy),
            )
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
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value + 1.0;

        assert!(!factory
            .nearly_eq(first_value)
            .matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equivalent_to_slightly_different_float() {
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value + 0.000_000_000_000_1;

        assert!(factory
            .nearly_eq(first_value)
            .matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equal_to_itself() {
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value;

        assert!(factory
            .nearly_eq(first_value)
            .matches_argument(&second_value));
    }

    #[test]
    fn float_is_not_nearly_equal_to_different_float_with_no_accuracy() {
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value + 1.0;
        let accuracy = 0.0;

        assert!(!factory
            .nearly_eq_with_accuracy(first_value, accuracy)
            .matches_argument(&second_value));
    }

    #[test]
    fn float_is_not_nearly_equal_to_different_float_with_same_accuracy_as_difference() {
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value + 0.1;
        let accuracy = 0.1;

        assert!(!factory
            .nearly_eq_with_accuracy(first_value, accuracy)
            .matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equal_to_different_float_low_accuracy() {
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value + 0.01;
        let accuracy = 0.1;

        assert!(factory
            .nearly_eq_with_accuracy(first_value, accuracy)
            .matches_argument(&second_value));
    }

    #[test]
    fn float_is_not_nearly_equal_to_different_float_with_highest_accuracy() {
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value + 0.1;
        let accuracy = 0.0;

        assert!(!factory
            .nearly_eq_with_accuracy(first_value, accuracy)
            .matches_argument(&second_value));
    }

    #[test]
    fn float_is_nearly_equal_to_itself_with_highest_accuracy() {
        let factory = Argument::internal_new();
        let first_value = 3.0;
        let second_value = first_value;
        let accuracy = 0.0;

        assert!(factory
            .nearly_eq_with_accuracy(first_value, accuracy)
            .matches_argument(&second_value));
    }
}
