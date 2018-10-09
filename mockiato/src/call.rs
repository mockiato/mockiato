use crate::arguments::Arguments;
use crate::expected_calls::ExpectedCalls;
use crate::return_value::{self, DefaultReturnValue, ReturnValue};
use std::fmt::{self, Display};
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

pub struct CallBuilder<'mock, A, R>
where
    A: Arguments<'mock>,
{
    call: Arc<RwLock<Call<'mock, A, R>>>,
}

impl<'mock, A, R> CallBuilder<'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub fn will_return(&self, return_value: R) -> &Self
    where
        R: Clone + 'mock,
    {
        let mut call = self.call.write().expect("unable to write call");

        call.deref_mut().return_value = Some(Box::new(return_value::Cloned(return_value)));

        self
    }

    pub fn times<E>(&self, expected_calls: E) -> &Self
    where
        E: Into<ExpectedCalls>,
    {
        let mut call = self.call.write().expect("unable to write call");

        call.deref_mut().expected_calls = expected_calls.into();

        self
    }

    pub(crate) fn new(call: Arc<RwLock<Call<'mock, A, R>>>) -> Self {
        CallBuilder { call }
    }
}

pub struct Call<'mock, A, R>
where
    A: Arguments<'mock>,
{
    expected_calls: ExpectedCalls,
    actual_number_of_calls: u64,
    matcher: A::Matcher,
    return_value: Option<Box<dyn ReturnValue<'mock, A, R> + 'mock>>,
}

impl<'mock, A, R> Call<'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub(crate) fn new(matcher: A::Matcher) -> Self {
        Call {
            expected_calls: ExpectedCalls::default(),
            actual_number_of_calls: 0,
            matcher,
            return_value: R::default_return_value(),
        }
    }

    pub(crate) fn call(&mut self, arguments: A) -> R {
        self.actual_number_of_calls += 1;

        match self.return_value {
            Some(ref return_value) => return_value.return_value(&arguments),
            None => panic!("No return value was specified"),
        }
    }
}

impl<'mock, A, R> Display for Call<'mock, A, R>
where
    A: Arguments<'mock>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({:?}) {:?} -> {:?}",
            self.expected_calls, self.matcher, self.return_value
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::matcher::IntoArgumentMatcher;

    #[test]
    #[should_panic(expected = "No return value was specified")]
    fn call_panics_if_no_return_value_is_specified() {
        let mut call: Call<((),), String> = Call::new((().into_argument_matcher(),));

        call.call(((),));
    }
}
