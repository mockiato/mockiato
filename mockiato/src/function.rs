use crate::arguments::Arguments;
use crate::call::{Call, CallBuilder};

pub struct MockedFunction<'mock, A, R>
where
    A: Arguments<'mock>,
{
    name: &'static str,
    calls: Vec<Call<'mock, A, R>>,
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

    pub fn add_expected_call(&mut self, matcher: A::Matcher) -> CallBuilder<'_, 'mock, A, R> {
        let call = Call::new(matcher);

        self.calls.push(call);

        CallBuilder::new(self.calls.last_mut().unwrap())
    }
}
