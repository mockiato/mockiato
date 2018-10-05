use crate::arguments::Arguments;
use crate::debug::MaybeDebug;
use std::fmt::{self, Debug};

pub trait DefaultReturnValue {
    fn default_return_value<'mock, A>() -> Option<Box<dyn ReturnValue<'mock, A, Self> + 'mock>>
    where
        Self: Sized,
        A: Arguments<'mock>;
}

impl<T> DefaultReturnValue for T {
    default fn default_return_value<'mock, A>() -> Option<Box<dyn ReturnValue<'mock, A, T> + 'mock>>
    where
        Self: Sized,
        A: Arguments<'mock>,
    {
        None
    }
}

impl DefaultReturnValue for () {
    fn default_return_value<'mock, A>() -> Option<Box<dyn ReturnValue<'mock, A, ()> + 'mock>>
    where
        Self: Sized,
        A: Arguments<'mock>,
    {
        Some(Box::new(Cloned(())))
    }
}

pub trait ReturnValue<'mock, A, R>: Debug
where
    A: Arguments<'mock>,
{
    fn return_value(&self, input: &A) -> R;
}

pub struct Cloned<T>(pub(crate) T)
where
    T: Clone + MaybeDebug;

impl<'mock, R> Debug for Cloned<R>
where
    R: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.0, f)
    }
}

impl<'mock, A, R> ReturnValue<'mock, A, R> for Cloned<R>
where
    A: Arguments<'mock>,
    R: Clone,
{
    fn return_value(&self, _: &A) -> R {
        self.0.clone()
    }
}

#[derive(Debug)]
pub struct Panic(Option<&'static str>);

impl<'mock, A, R> ReturnValue<'mock, A, R> for Panic
where
    A: Arguments<'mock>,
{
    fn return_value(&self, _: &A) -> R {
        match self.0 {
            Some(message) => panic!(message),
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic(expected = "<panic message>")]
    fn test_panic_panicks() {
        let panic = Panic(Some("<panic message>"));

        ReturnValue::<((),), ()>::return_value(&panic, &((),));
    }

    #[test]
    fn test_cloned_returns_expected_value() {
        let cloned = Cloned(String::from("foo"));

        assert_eq!(String::from("foo"), cloned.return_value(&((),)));
    }
}
