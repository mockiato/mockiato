pub(crate) use self::cloned::*;
pub(crate) use self::panic::*;

use crate::internal::ArgumentsMatcher;
use std::fmt::{Debug, Display};

mod cloned;
mod panic;

pub trait ReturnValueGenerator<A, R>: Display + Debug
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, input: <A as ArgumentsMatcher<'_>>::Arguments) -> R;
}
