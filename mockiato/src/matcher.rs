use crate::arguments::Arguments;

pub trait ArgumentMatcher<T> {
    fn matches_argument(&self, input: &T) -> bool;
}

impl<T> ArgumentMatcher<T> for T
where
    T: PartialEq,
{
    fn matches_argument(&self, input: &T) -> bool {
        self == input
    }
}

pub trait ArgumentsMatcher<'mock, A>
where
    A: Arguments<'mock> + ?Sized,
{
    fn matches_call(&self, input: &A) -> bool;
}

impl<'mock, A> ArgumentsMatcher<'mock, (A,)> for (Box<ArgumentMatcher<A> + 'mock>,) {
    fn matches_call(&self, input: &(A,)) -> bool {
        self.0.matches_argument(&input.0)
    }
}

impl<'mock, A, B> ArgumentsMatcher<'mock, (A, B)>
    for (
        Box<ArgumentMatcher<A> + 'mock>,
        Box<ArgumentMatcher<B> + 'mock>,
    )
{
    fn matches_call(&self, input: &(A, B)) -> bool {
        self.0.matches_argument(&input.0) && self.1.matches_argument(&input.1)
    }
}
