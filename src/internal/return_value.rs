use crate::internal::fmt::{DisplayOption, MaybeDebug};
use crate::internal::ArgumentsMatcher;
use std::fmt::{self, Debug, Display};
use std::rc::Rc;

pub trait DefaultReturnValue<A>: Sized {
    fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, Self>>>;
}

impl<A, T> DefaultReturnValue<A> for T
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    default fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, T>>> {
        None
    }
}

impl<A> DefaultReturnValue<A> for ()
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, ()>>> {
        Some(Rc::new(Cloned(())))
    }
}

pub trait ReturnValueGenerator<A, R>: Display + Debug
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, input: <A as ArgumentsMatcher<'_>>::Arguments) -> R;
}

pub struct Cloned<T>(pub(crate) T);

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

impl<A, R> ReturnValueGenerator<A, R> for Cloned<R>
where
    R: Clone,
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, _: <A as ArgumentsMatcher<'_>>::Arguments) -> R {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct Panic(pub(crate) Option<&'static str>);

impl Display for Panic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "panic!({})", DisplayOption(self.0.as_ref()))
    }
}

impl<A, R> ReturnValueGenerator<A, R> for Panic
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, _: <A as ArgumentsMatcher<'_>>::Arguments) -> R {
        match self.0 {
            Some(message) => panic!(message),
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::arguments::ArgumentsMock;
    use crate::internal::matcher::ArgumentsMatcherMock;

    #[test]
    #[should_panic(expected = "<panic message>")]
    fn test_panic_panicks() {
        let panic = Panic(Some("<panic message>"));

        ReturnValueGenerator::<ArgumentsMatcherMock, ()>::generate_return_value(
            &panic,
            ArgumentsMock,
        );
    }

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
