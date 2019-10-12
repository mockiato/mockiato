use crate::matcher::ArgumentsMatcher;
use crate::return_value::ReturnValueGenerator;
use std::rc::Rc;

#[cfg(rustc_is_nightly)]
use crate::return_value::Cloned;

pub(crate) trait DefaultReturnValue<A>: Sized {
    fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, Self>>> {
        None
    }
}

#[cfg(not(rustc_is_nightly))]
impl<A, T> DefaultReturnValue<A> for T {}

#[cfg(rustc_is_nightly)]
impl<A, T> DefaultReturnValue<A> for T {
    default fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, Self>>> {
        None
    }
}

#[cfg(rustc_is_nightly)]
impl<A> DefaultReturnValue<A> for ()
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, ()>>> {
        Some(Rc::new(Cloned(())))
    }
}
