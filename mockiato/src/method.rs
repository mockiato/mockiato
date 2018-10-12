use crate::arguments::Arguments;
use crate::method_call::{MethodCall, MethodCallBuilder};

pub struct Method<'mock, A, R>
where
    A: Arguments<'mock>,
{
    name: &'static str,
    calls: Vec<MethodCall<'mock, A, R>>,
}

impl<'mock, A, R> Method<'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: Default::default(),
        }
    }

    pub fn add_expected_call(&mut self, matcher: A::Matcher) -> MethodCallBuilder<'_, 'mock, A, R> {
        let call = MethodCall::new(matcher);

        self.calls.push(call);

        MethodCallBuilder::new(self.calls.last_mut().unwrap())
    }
}
