use super::ReturnValueGenerator;
use crate::fmt::MaybeDebug;
use crate::matcher::ArgumentsMatcher;
use std::cell::RefCell;
use std::fmt::{self, Debug, Display};

pub(crate) struct Once<T>(RefCell<Option<T>>);

impl<T> Once<T> {
    pub(crate) fn new(value: T) -> Self {
        Self(RefCell::new(Some(value)))
    }
}

impl<A, R> ReturnValueGenerator<A, R> for Once<R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, _: <A as ArgumentsMatcher<'_>>::Arguments) -> R {
        self.0
            .borrow_mut()
            .take()
            .expect("This value was already returned")
    }

    fn can_return_more_than_once(&self) -> bool {
        false
    }
}

impl<R> Display for Once<R>
where
    R: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<R> Debug for Once<R>
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
    use crate::arguments::ArgumentsMock;
    use crate::matcher::ArgumentsMatcherMock;

    #[test]
    fn returns_expected_value() {
        let return_value = Once::new(String::from("foo"));

        assert_eq!(
            String::from("foo"),
            ReturnValueGenerator::<ArgumentsMatcherMock, String>::generate_return_value(
                &return_value,
                ArgumentsMock
            )
        );
    }

    #[test]
    #[should_panic]
    fn panics_when_called_more_than_once() {
        let return_value = Once::new(String::from("foo"));

        assert_eq!(
            String::from("foo"),
            ReturnValueGenerator::<ArgumentsMatcherMock, _>::generate_return_value(
                &return_value,
                ArgumentsMock
            )
        );

        ReturnValueGenerator::<ArgumentsMatcherMock, _>::generate_return_value(
            &return_value,
            ArgumentsMock,
        );
    }
}
