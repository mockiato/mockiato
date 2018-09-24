use crate::arguments::Arguments;
use crate::expected_calls::ExpectedCalls;
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
}

pub struct Calls<'mock, A, R>
where
    A: Arguments<'mock>,
{
    name: &'static str,
    calls: Arc<RwLock<Vec<Arc<RwLock<Call<'mock, A, R>>>>>>,
}

impl<'mock, A, R> Calls<'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub fn new(name: &'static str) -> Self {
        Calls {
            name,
            calls: Default::default(),
        }
    }

    #[must_use]
    pub fn expect(&self, matcher: A::Matcher) -> CallBuilder<'mock, A, R> {
        let call = Arc::new(RwLock::new(Call::new(matcher)));

        self.calls
            .write()
            .expect("unable to write calls")
            .push(call.clone());

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
    pub fn new(matcher: A::Matcher) -> Self {
        Call {
            expected_calls: ExpectedCalls::Any,
            actual_number_of_calls: 0,
            matcher,
            return_value: None,
        }
    }
}
