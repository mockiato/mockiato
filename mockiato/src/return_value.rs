use crate::arguments::Arguments;

pub trait ReturnValue<'mock, A, R>
where
    A: Arguments<'mock>,
{
    fn return_value(&self, input: &A) -> R;
}

pub struct Cloned<T>(pub(crate) T)
where
    T: Clone;

impl<'mock, A, R> ReturnValue<'mock, A, R> for Cloned<R>
where
    A: Arguments<'mock>,
    R: Clone,
{
    fn return_value(&self, _: &A) -> R {
        self.0.clone()
    }
}

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
