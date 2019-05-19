use crate::internal::matcher::ArgumentsMatcher;
use crate::internal::return_value::{Cloned, ReturnValueGenerator};
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
