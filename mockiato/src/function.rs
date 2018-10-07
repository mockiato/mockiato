use crate::arguments::Arguments;
use crate::call::{Call, CallBuilder};
use crate::expected_calls::ExpectedCalls;
use crate::return_value::{self, DefaultReturnValue, ReturnValue};
use std::ops::DerefMut;
use std::sync::{Arc, RwLock};

pub struct MockedFunction<'mock, A, R>
where
    A: Arguments<'mock>,
{
    name: &'static str,
    calls: Arc<RwLock<Vec<Arc<RwLock<Call<'mock, A, R>>>>>>,
}

impl<'mock, A, R> MockedFunction<'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub fn new(name: &'static str) -> Self {
        MockedFunction {
            name,
            calls: Default::default(),
        }
    }

    pub fn expect(&self, matcher: A::Matcher) -> CallBuilder<'mock, A, R> {
        let call = self.add_expected_call(Call::new(matcher));

        CallBuilder::new(call)
    }

    fn add_expected_call(&self, call: Call<'mock, A, R>) -> Arc<RwLock<Call<'mock, A, R>>> {
        let call = Arc::new(RwLock::new(call));

        self.calls
            .write()
            .expect("unable to write calls")
            .push(call.clone());

        call
    }
}
