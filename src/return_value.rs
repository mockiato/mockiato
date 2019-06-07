pub(crate) use self::cloned::*;
pub(crate) use self::once::*;
pub(crate) use self::panic::*;

use crate::matcher::ArgumentsMatcher;
use std::fmt::{Debug, Display};
use std::rc::Rc;

mod cloned;
mod once;
mod panic;

pub(crate) trait DefaultReturnValue<A>: Sized {
    fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, Self>>> {
        None
    }
}

impl<A, T> DefaultReturnValue<A> for T where A: for<'args> ArgumentsMatcher<'args> {}

#[cfg(rustc_is_nightly)]
impl<A> DefaultReturnValue<A> for ()
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn default_return_value() -> Option<Rc<dyn ReturnValueGenerator<A, ()>>> {
        Some(Rc::new(Cloned(())))
    }
}

pub(crate) trait ReturnValueGenerator<A, R>: Display + Debug
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, input: <A as ArgumentsMatcher<'_>>::Arguments) -> R;
}
