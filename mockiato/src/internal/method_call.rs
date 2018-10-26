use crate::internal::expected_calls::ExpectedCalls;
use crate::internal::fmt::DisplayTimes;
use crate::internal::matcher::ArgumentsMatcher;
use crate::internal::return_value::{self, DefaultReturnValue, ReturnValueGenerator};
use crate::internal::DisplayOption;
use std::cell::RefCell;
use std::fmt::{self, Display};

/// Configures an expected method call.
/// This builder is returned from the `expect_*` methods on a generated mock.
pub struct MethodCallBuilder<'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    call: &'a mut MethodCall<A, R>,
}

impl<'a, A, R> MethodCallBuilder<'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    /// Defines the return value for this method.
    /// The value must be [`Clone`]able.g
    pub fn returns(&mut self, return_value: R) -> &mut Self
    where
        R: Clone + 'static,
    {
        self.call.return_value = Some(Box::new(return_value::Cloned(return_value)));
        self
    }

    /// Defines that this method panics.
    pub fn panics(&mut self) -> &mut Self {
        self.call.return_value = Some(Box::new(return_value::Panic(None)));
        self
    }

    /// Defines that this method panics with a message.
    pub fn panics_with_message(&mut self, message: &'static str) -> &mut Self {
        self.call.return_value = Some(Box::new(return_value::Panic(Some(message))));
        self
    }

    /// Defines how often this method should be called.
    pub fn times<E>(&mut self, expected_calls: E) -> &mut Self
    where
        E: Into<ExpectedCalls>,
    {
        self.call.expected_calls = expected_calls.into();
        self
    }

    pub(crate) fn new(call: &'a mut MethodCall<A, R>) -> Self {
        Self { call }
    }
}

#[derive(Debug)]
pub struct MethodCall<A, R> {
    expected_calls: ExpectedCalls,
    actual_number_of_calls: RefCell<u64>,
    matcher: A,
    return_value: Option<Box<dyn ReturnValueGenerator<A, R>>>,
}

impl<A, R> MethodCall<A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    pub(crate) fn new(matcher: A) -> Self {
        Self {
            expected_calls: ExpectedCalls::default(),
            actual_number_of_calls: Default::default(),
            matcher,
            return_value: R::default_return_value(),
        }
    }

    pub(crate) fn call(&self, arguments: <A as ArgumentsMatcher>::Arguments) -> R {
        *self.actual_number_of_calls.borrow_mut() += 1;

        match self.return_value {
            Some(ref return_value) => return_value.generate_return_value(arguments),
            None => panic!("No return value was specified"),
        }
    }

    pub(crate) fn was_called_expected_number_of_times(&self) -> bool {
        self.expected_calls
            .matches_value(*self.actual_number_of_calls.borrow())
    }

    pub(crate) fn matches_expected_arguments<'a>(
        &self,
        arguments: &<A as ArgumentsMatcher<'a>>::Arguments,
    ) -> bool {
        self.matcher.matches_arguments(arguments)
    }
}

impl<'mock, A, R> Display for MethodCall<A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} -> {} {}, was called {}",
            self.matcher,
            DisplayOption(self.return_value.as_ref()),
            self.expected_calls,
            DisplayTimes(*self.actual_number_of_calls.borrow())
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::arguments::ArgumentsMock;
    use crate::internal::matcher::ArgumentsMatcherMock;
    use std::cell::RefCell;
    use std::fmt::Debug;
    use std::thread::panicking;

    #[derive(Debug)]
    struct ReturnValueGeneratorMock<R>
    where
        R: Clone + Debug,
    {
        generate_return_value_was_called: RefCell<bool>,
        return_value: Option<R>,
    }

    impl<R> ReturnValueGeneratorMock<R>
    where
        R: Clone + Debug,
    {
        fn new(return_value: Option<R>) -> Self {
            Self {
                return_value,
                generate_return_value_was_called: Default::default(),
            }
        }
    }

    impl<R> Display for ReturnValueGeneratorMock<R>
    where
        R: Clone + Debug,
    {
        fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!();
        }
    }

    impl<A, R> ReturnValueGenerator<A, R> for ReturnValueGeneratorMock<R>
    where
        A: for<'args> ArgumentsMatcher<'args>,
        R: Clone + Debug,
    {
        fn generate_return_value(&self, _input: <A as ArgumentsMatcher>::Arguments) -> R {
            *self.generate_return_value_was_called.borrow_mut() = true;

            self.return_value
                .as_ref()
                .expect("Return value was not specified for mock")
                .clone()
        }
    }

    impl<R> Drop for ReturnValueGeneratorMock<R>
    where
        R: Clone + Debug,
    {
        fn drop(&mut self) {
            if !panicking() && self.return_value.is_some() {
                assert!(
                    *self.generate_return_value_was_called.borrow(),
                    "generate_return_value_was_called() was never called"
                );
            }
        }
    }

    #[test]
    #[should_panic(expected = "No return value was specified")]
    fn call_panics_if_no_return_value_is_specified() {
        let call: MethodCall<_, String> = MethodCall::new(ArgumentsMatcherMock::new(None));

        call.call(ArgumentsMock);
    }

    #[test]
    fn call_uses_return_value() {
        let mut call: MethodCall<_, String> = MethodCall::new(ArgumentsMatcherMock::new(None));

        call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(Some(String::from(
            "foo",
        )))));

        let return_value = call.call(ArgumentsMock);

        assert_eq!(String::from("foo"), return_value);
    }

    #[test]
    fn was_called_expected_number_of_times_returns_true() {
        let mut call: MethodCall<_, ()> = MethodCall::new(ArgumentsMatcherMock::new(None));
        call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(Some(()))));
        call.expected_calls = 4.into();

        call.call(ArgumentsMock);
        call.call(ArgumentsMock);
        call.call(ArgumentsMock);
        call.call(ArgumentsMock);

        assert!(call.was_called_expected_number_of_times());
    }

    #[test]
    fn was_called_expected_number_of_times_returns_false() {
        let call: MethodCall<_, ()> = {
            let mut call = MethodCall::new(ArgumentsMatcherMock::new(None));
            call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(Some(()))));
            call.expected_calls = (2..).into();
            call
        };

        call.call(ArgumentsMock);

        assert!(!call.was_called_expected_number_of_times());
    }

    #[test]
    fn matches_expected_arguments_returns_true() {
        let call: MethodCall<_, ()> = {
            let mut call = MethodCall::new(ArgumentsMatcherMock::new(Some(true)));
            call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(None)));
            call
        };

        assert!(call.matches_expected_arguments(&ArgumentsMock));
    }

    #[test]
    fn matches_expected_arguments_returns_false() {
        let call: MethodCall<_, ()> = {
            let mut call = MethodCall::new(ArgumentsMatcherMock::new(Some(false)));
            call.return_value = Some(Box::new(ReturnValueGeneratorMock::new(None)));
            call
        };

        assert!(!call.matches_expected_arguments(&ArgumentsMock));
    }
}
