use super::ArgumentMatcher;
use crate::internal::fmt::MaybeDebugWrapper;
use std::fmt::{self, Debug};

/// Creates a new `ArgumentMatcher` that matches [`Vec`]s and [`slice`]s
/// while disregarding the exact order of the elements.
///
/// [`slice`]: https://doc.rust-lang.org/std/primitive.slice.html
pub fn unordered_vec_eq<T>(vec: Vec<T>) -> UnorderedVecArgumentMatcher<T>
where
    T: 'static,
{
    UnorderedVecArgumentMatcher(vec)
}

pub struct UnorderedVecArgumentMatcher<T>(Vec<T>)
where
    T: 'static;

impl<T> Debug for UnorderedVecArgumentMatcher<T>
where
    T: 'static,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&MaybeDebugWrapper(&self.0), f)
    }
}

impl<T, U> ArgumentMatcher<Vec<U>> for UnorderedVecArgumentMatcher<T>
where
    T: PartialEq<U> + 'static,
    U: PartialEq<T>,
{
    fn matches_argument(&self, input: &Vec<U>) -> bool {
        compare_slices_unordered(&self.0, input.as_slice())
    }
}

impl<'a, T, U> ArgumentMatcher<&'a [U]> for UnorderedVecArgumentMatcher<T>
where
    T: PartialEq<U> + 'static,
    U: PartialEq<T>,
{
    fn matches_argument(&self, input: &&'a [U]) -> bool {
        compare_slices_unordered(&self.0, *input)
    }
}

impl<'a, T, U> ArgumentMatcher<&'a mut [U]> for UnorderedVecArgumentMatcher<T>
where
    T: PartialEq<U> + 'static,
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
    all_slice_elements_are_found_in_other(expected, actual)
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
