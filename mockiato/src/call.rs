use crate::arguments::ArgumentsMatcher;
use crate::return_value::{CloneValue, ReturnValue};
use std::fmt;
use std::sync::{Arc, RwLock};

///
/// This struct is public because it is used in generated code.
/// Never use this struct directly.
///
#[derive(Debug, Clone)]
#[doc(hidden)]
pub struct Calls<'a, I: 'a, O: 'a, A>
where
    A: ArgumentsMatcher<I>,
{
    name: &'static str,
    calls: Arc<RwLock<Vec<Call<'a, I, O, A>>>>,
}

impl<'a, I: 'a, O: 'a, A> Calls<'a, I, O, A>
where
    A: ArgumentsMatcher<I>,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: Default::default(),
        }
    }

    pub fn verify(&self) {
        let calls = self
            .calls
            .read()
            .expect("unable to aquire read lock for calls");

        for call in calls.iter() {
            call.verify();
        }
    }

    pub fn add(&self, call: Call<'a, I, O, A>) {
        self.calls
            .write()
            .expect("unable to aquire write lock for calls")
            .push(call);
    }

    pub fn call(&self, args: I) -> O {
        let mut calls = self
            .calls
            .write()
            .expect("unable to aquire read lock for calls");

        let matching_call = calls
            .iter_mut()
            .find(|call| call.matcher.matches_args(&args))
            .expect(&format!(
                "

Unexpected call to {}

",
                self.name,
            ));

        matching_call.record_call();

        matching_call
            .return_value
            .as_ref()
            .expect(&format!(
                "

Return value was not specified for {}

",
                self.name,
            )).return_value(&args)
    }
}

pub struct Call<'a, I: 'a, O: 'a, A>
where
    A: ArgumentsMatcher<I>,
{
    name: &'static str,
    matcher: A,
    return_value: Option<Box<dyn ReturnValue<I, O> + 'a>>,
    expected_calls: u64,
    actual_calls: u64,
}

impl<'a, I, O, A> fmt::Debug for Call<'a, I, O, A>
where
    A: ArgumentsMatcher<I>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Call").finish()
    }
}

impl<'a, I: 'a, O: 'a, A> Call<'a, I, O, A>
where
    A: ArgumentsMatcher<I>,
{
    #[doc(hidden)]
    pub fn new(name: &'static str, matcher: A) -> Self {
        Self {
            name,
            matcher,
            return_value: None,
            expected_calls: 1,
            actual_calls: 0,
        }
    }

    fn record_call(&mut self) {
        self.actual_calls += 1;
    }

    fn verify(&self) {
        if self.actual_calls != self.expected_calls {
            panic!(
                "

`{}` was expected to be called {} times.
Was actually called {} times.

",
                self.name, self.expected_calls, self.actual_calls,
            );
        }
    }

    pub fn times(&mut self, times: u64) -> &mut Self {
        self.expected_calls = times;
        self
    }

    pub fn will_return(&mut self, value: O) -> &mut Self
    where
        O: Clone + Sync,
    {
        self.return_value = Some(Box::new(CloneValue(value)));
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
