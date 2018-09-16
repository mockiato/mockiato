use crate::private;

pub trait CallMatcher<I>: private::Sealed {
    fn matches_call(&self, input: &I) -> bool;
}

pub fn args<'a, I: 'a>(args: I) -> Box<dyn CallMatcher<I> + 'a>
where
    I: PartialEq,
{
    Box::new(PartialEqMatcher(args))
}

struct PartialEqMatcher<T>(pub(crate) T)
where
    T: PartialEq;

impl<T> private::Sealed for PartialEqMatcher<T> where T: PartialEq {}

impl<I> CallMatcher<I> for PartialEqMatcher<I>
where
    I: PartialEq,
{
    fn matches_call(&self, input: &I) -> bool {
        input == &self.0
    }
}
