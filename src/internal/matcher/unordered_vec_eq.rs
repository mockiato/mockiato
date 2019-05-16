use super::ArgumentMatcher;
use crate::internal::argument::Argument;
use crate::internal::fmt::MaybeDebugWrapper;
use nameof::name_of;
use std::fmt::{self, Debug, Display};

impl Argument {
    /// Creates an argument matcher that matches against [`Vec`]s and [`slice`]s
    /// while disregarding the exact order of the elements.
    ///
    /// Requires the elements to implement [`PartialEq`].
    ///
    /// # Examples
    /// ```
    /// use mockiato::mockable;
    ///
    /// #[cfg_attr(test, mockable)]
    /// trait MessageSender {
    ///     fn send_message(&self, messages: &[&str]);
    /// }
    ///
    /// let mut sender = MessageSenderMock::new();
    /// let message = "Hello World";
    /// sender.expect_send_message(|a| a.unordered_vec_eq(vec!["foo", "bar", "baz"]));
    /// sender.send_message(&["baz", "bar", "foo"]);
    /// ```
    ///
    /// [`slice`]: https://doc.rust-lang.org/std/primitive.slice.html
    pub fn unordered_vec_eq<T>(&self, vec: Vec<T>) -> UnorderedVecArgumentMatcher<T> {
        UnorderedVecArgumentMatcher(vec)
    }
}

pub struct UnorderedVecArgumentMatcher<T>(Vec<T>);

impl<T> Display for UnorderedVecArgumentMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} in any order", &MaybeDebugWrapper(&self.0))
    }
}

impl<T> Debug for UnorderedVecArgumentMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(name_of!(type UnorderedVecArgumentMatcher<T>))
            .field(&MaybeDebugWrapper(&self.0))
            .finish()
    }
}

impl<T, U> ArgumentMatcher<Vec<U>> for UnorderedVecArgumentMatcher<T>
where
    T: PartialEq<U>,
    U: PartialEq<T>,
{
    fn matches_argument(&self, input: &Vec<U>) -> bool {
        compare_slices_unordered(&self.0, input.as_slice())
    }
}

impl<'a, T, U> ArgumentMatcher<&'a [U]> for UnorderedVecArgumentMatcher<T>
where
    T: PartialEq<U>,
    U: PartialEq<T>,
{
    fn matches_argument(&self, input: &&'a [U]) -> bool {
        compare_slices_unordered(&self.0, *input)
    }
}

impl<'a, T, U> ArgumentMatcher<&'a mut [U]> for UnorderedVecArgumentMatcher<T>
where
    T: PartialEq<U>,
    U: PartialEq<T>,
{
    fn matches_argument(&self, input: &&'a mut [U]) -> bool {
        compare_slices_unordered(&self.0, *input)
    }
}

fn compare_slices_unordered<T, U>(expected: &[T], actual: &[U]) -> bool
where
    T: PartialEq<U>,
    U: PartialEq<T>,
{
    expected.len() == actual.len()
        && all_slice_elements_are_found_in_other(expected, actual)
        && all_slice_elements_are_found_in_other(actual, expected)
}

/// Checks every element of `left` exists in `right`
fn all_slice_elements_are_found_in_other<T, U>(left: &[T], right: &[U]) -> bool
where
    T: PartialEq<U>,
    U: PartialEq<T>,
{
    left.iter()
        .all(|element| right.iter().any(|element_2| element_2 == element))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn accepts_empty_slices() {
        assert!(compare_slices_unordered::<usize, usize>(&[], &[]));
    }

    #[test]
    fn rejects_slices_with_different_lengths() {
        assert!(!compare_slices_unordered(&[1], &[1, 2]));
    }

    #[test]
    fn works_with_duplicated_items() {
        assert!(compare_slices_unordered(
            &["foo", "foo", "bar"],
            &["foo", "bar", "foo"]
        ))
    }

    #[test]
    fn does_not_work_with_different_lengths() {
        assert!(!compare_slices_unordered(
            &["foo", "foo", "bar"],
            &["foo", "bar"]
        ))
    }

    #[test]
    fn rejects_slices_that_have_different_items() {
        assert!(!compare_slices_unordered(
            &["foo", "bar", "baz"],
            &["bar", "foo", "foo"]
        ))
    }

    #[test]
    fn rejects_when_actual_slice_has_extra_elements() {
        assert!(!compare_slices_unordered(&["foo"], &["foo", "bar"]))
    }
}
