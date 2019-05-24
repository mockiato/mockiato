use super::ReturnValueGenerator;
use crate::internal::fmt::MaybeDebug;
use crate::internal::ArgumentsMatcher;
use std::fmt::{self, Debug, Display};

pub struct Cloned<T>(pub(crate) T);

impl<A, R> ReturnValueGenerator<A, R> for Cloned<R>
where
    R: Clone,
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, _: <A as ArgumentsMatcher<'_>>::Arguments) -> R {
        self.0.clone()
    }
}

impl<R> Display for Cloned<R>
where
    R: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<R> Debug for Cloned<R>
where
    R: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::arguments::ArgumentsMock;
    use crate::internal::matcher::ArgumentsMatcherMock;

    #[test]
    fn test_cloned_returns_expected_value() {
        let cloned = Cloned(String::from("foo"));

        assert_eq!(
            String::from("foo"),
            ReturnValueGenerator::<ArgumentsMatcherMock, String>::generate_return_value(
                &cloned,
                ArgumentsMock
            )
        );
    }
}
