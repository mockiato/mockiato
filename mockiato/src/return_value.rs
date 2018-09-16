use crate::private;

pub(crate) trait ReturnValue<I, O>: private::Sealed {
    fn return_value(&self, input: &I) -> O;
}

pub(crate) struct CloneValue<T>(pub(crate) T)
where
    T: Clone;

impl<O> private::Sealed for CloneValue<O> where O: Clone {}

impl<I, O> ReturnValue<I, O> for CloneValue<O>
where
    O: Clone,
{
    fn return_value(&self, _: &I) -> O {
        self.0.clone()
    }
}
