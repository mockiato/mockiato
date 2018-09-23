use crate::arguments::Arguments;
use crate::return_value::ReturnValue;
use std::sync::{Arc, RwLock};

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
}

pub struct Call<'mock, A, R>
where
    A: Arguments<'mock>,
{
    expected_calls: u64,
    actual_calls: u64,
    matcher: A::Matcher,
    return_value: Option<Box<dyn ReturnValue<'mock, A, R>>>,
}

impl<'mock, A, R> Call<'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub fn new(matcher: A::Matcher) -> Self {
        Call {
            expected_calls: 1,
            actual_calls: 0,
            matcher,
            return_value: None,
        }
    }
}
