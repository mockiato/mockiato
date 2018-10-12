use crate::arguments::Arguments;
use crate::expected_calls::ExpectedCalls;
use crate::return_value::{self, DefaultReturnValue, ReturnValueGenerator};
use std::fmt::{self, Display};

pub struct MethodCallBuilder<'a, 'mock, A, R>
where
    A: Arguments<'mock>,
{
    call: &'a mut MethodCall<'mock, A, R>,
}

impl<'a, 'mock, A, R> MethodCallBuilder<'a, 'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub fn returns(&mut self, return_value: R) -> &mut Self
    where
        R: Clone + 'mock,
    {
        self.call.return_value = Some(Box::new(return_value::Cloned(return_value)));
        self
    }

    pub fn times<E>(&mut self, expected_calls: E) -> &mut Self
    where
        E: Into<ExpectedCalls>,
    {
        self.call.expected_calls = expected_calls.into();
        self
    }

    pub(crate) fn new(call: &'a mut MethodCall<'mock, A, R>) -> Self {
        Self { call }
    }
}

pub struct MethodCall<'mock, A, R>
where
    A: Arguments<'mock>,
{
    expected_calls: ExpectedCalls,
    actual_number_of_calls: u64,
    matcher: A::Matcher,
    return_value: Option<Box<dyn ReturnValueGenerator<'mock, A, R> + 'mock>>,
}

impl<'mock, A, R> MethodCall<'mock, A, R>
where
    A: Arguments<'mock>,
{
    pub(crate) fn new(matcher: A::Matcher) -> Self {
        Self {
            expected_calls: ExpectedCalls::default(),
            actual_number_of_calls: 0,
            matcher,
            return_value: R::default_return_value(),
        }
    }

    pub(crate) fn call(&mut self, arguments: A) -> R {
        self.actual_number_of_calls += 1;

        match self.return_value {
            Some(ref return_value) => return_value.generate_return_value(&arguments),
            None => panic!("No return value was specified"),
        }
    }

    pub(crate) fn was_called_expected_number_of_times(&self) -> bool {
        self.expected_calls
            .matches_value(self.actual_number_of_calls)
    }
}

impl<'mock, A, R> Display for MethodCall<'mock, A, R>
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
    use std::cell::RefCell;
    use std::fmt::Debug;

    #[derive(Debug)]
    struct ReturnValueGeneratorMock<R>
    where
        R: Clone + Debug,
    {
        generate_return_value_was_called: RefCell<bool>,
        return_value: R,
    }

    impl<R> ReturnValueGeneratorMock<R>
    where
        R: Clone + Debug,
    {
        fn new(return_value: R) -> Self {
            Self {
                return_value,
                generate_return_value_was_called: Default::default(),
            }
        }
    }

    impl<'mock, A, R> ReturnValueGenerator<'mock, A, R> for ReturnValueGeneratorMock<R>
    where
        A: Arguments<'mock>,
        R: Clone + Debug,
    {
        fn generate_return_value(&self, _input: &A) -> R {
            *self.generate_return_value_was_called.borrow_mut() = true;
            self.return_value.clone()
        }
    }

    impl<R> Drop for ReturnValueGeneratorMock<R>
    where
        R: Clone + Debug,
    {
        fn drop(&mut self) {
            assert!(
                *self.generate_return_value_was_called.borrow(),
                "generate_return_value_was_called() was never called"
            );
        }
    }

    #[test]
    #[should_panic(expected = "No return value was specified")]
    fn call_panics_if_no_return_value_is_specified() {
        let mut call: MethodCall<((),), String> = MethodCall::new((().into_argument_matcher(),));

        call.call(((),));
    }

    #[test]
    fn call_uses_return_value() {
        let mut call: MethodCall<((),), String> = MethodCall::new((().into_argument_matcher(),));

        call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(String::from("foo"))));

        let return_value = call.call(((),));

        assert_eq!(String::from("foo"), return_value);
    }

    #[test]
    fn was_called_expected_number_of_times_returns_true() {
        let mut call: MethodCall<((),), ()> = MethodCall::new((().into_argument_matcher(),));
        call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(())));
        call.expected_calls = 4.into();

        call.call(((),));
        call.call(((),));
        call.call(((),));
        call.call(((),));

        assert!(call.was_called_expected_number_of_times());
    }

    #[test]
    fn was_called_expected_number_of_times_returns_false() {
        let mut call: MethodCall<((),), ()> = MethodCall::new((().into_argument_matcher(),));
        call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(())));
        call.expected_calls = (2..).into();

        call.call(((),));

        assert!(!call.was_called_expected_number_of_times());
    }
}
