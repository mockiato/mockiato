use crate::internal::debug::MaybeDebug;
use crate::internal::matcher::ArgumentsMatcher;
use std::fmt::{self, Debug, Display};

pub trait DefaultReturnValue {
    fn default_return_value<'mock, A>(
    ) -> Option<Box<dyn ReturnValueGenerator<'mock, A, Self> + 'mock>>
    where
        Self: Sized,
        A: ArgumentsMatcher<'mock> + 'mock;
}

impl<T> DefaultReturnValue for T {
    default fn default_return_value<'mock, A>(
    ) -> Option<Box<dyn ReturnValueGenerator<'mock, A, T> + 'mock>>
    where
        Self: Sized,
        A: ArgumentsMatcher<'mock> + 'mock,
    {
        None
    }
}

impl DefaultReturnValue for () {
    fn default_return_value<'mock, A>(
    ) -> Option<Box<dyn ReturnValueGenerator<'mock, A, ()> + 'mock>>
    where
        Self: Sized,
        A: ArgumentsMatcher<'mock> + 'mock,
    {
        Some(Box::new(Cloned(())))
    }
}

pub trait ReturnValueGenerator<'mock, A, R>: Display + Debug
where
    A: ArgumentsMatcher<'mock> + 'mock,
{
    fn generate_return_value(&self, input: A::Arguments) -> R;
}

pub struct Cloned<T>(pub(crate) T)
where
    T: Clone + MaybeDebug;

impl<'mock, R> Display for Cloned<R>
where
    R: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<'mock, R> Debug for Cloned<R>
where
    R: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<'mock, A, R> ReturnValueGenerator<'mock, A, R> for Cloned<R>
where
    A: ArgumentsMatcher<'mock> + 'mock,
    R: Clone,
{
    fn generate_return_value(&self, _: A::Arguments) -> R {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct Panic(pub(crate) Option<&'static str>);

impl Display for Panic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "panic!(")?;

        if let Some(message) = self.0 {
            write!(f, "{:?}", message)?;
        }

        write!(f, ")")
    }
}

impl<'mock, A, R> ReturnValueGenerator<'mock, A, R> for Panic
where
    A: ArgumentsMatcher<'mock> + 'mock,
{
    fn generate_return_value(&self, _: A::Arguments) -> R {
        match self.0 {
            Some(message) => panic!(message),
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::matcher::ArgumentMatcher;

    /*#[test]
    #[should_panic(expected = "<panic message>")]
    fn test_panic_panicks() {
        let panic = Panic(Some("<panic message>"));
    
        ReturnValueGenerator::<(Box<dyn ArgumentMatcher<()>>,), ()>::generate_return_value(
            &panic,
            ((),),
        );
    }
    
    #[test]
    fn test_cloned_returns_expected_value() {
        let cloned = Cloned(String::from("foo"));
    
        assert_eq!(
            String::from("foo"),
            ReturnValueGenerator::<(Box<dyn ArgumentMatcher<()>>,), ()>::generate_return_value(
                &cloned,
                ((),)
            )
        );
    }*/
}
