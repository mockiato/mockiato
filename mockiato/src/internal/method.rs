// use crate::internal::arguments::DebugArguments;
use crate::internal::matcher::ArgumentsMatcher;
use crate::internal::method_call::{MethodCall, MethodCallBuilder};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Method<'mock, A, R>
where
    A: ArgumentsMatcher<'mock> + 'mock,
{
    name: &'static str,
    calls: Vec<MethodCall<'mock, A, R>>,
}

impl<'mock, A, R> Method<'mock, A, R>
where
    A: ArgumentsMatcher<'mock> + 'mock,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: Default::default(),
        }
    }

    pub fn add_expected_call(&mut self, matcher: A) -> MethodCallBuilder<'_, 'mock, A, R> {
        let call = MethodCall::new(matcher);

        self.calls.push(call);

        MethodCallBuilder::new(self.calls.last_mut().unwrap())
    }

    pub fn call_unwrap(&self, arguments: A::Arguments) -> R {
        match self.call(arguments) {
            Ok(return_value) => return_value,
            Err(err) => panic!("{}", err),
        }
    }

    fn call(&self, arguments: A::Arguments) -> Result<R, CallError<'_, 'mock, A, R>> {
        let matching_method_calls = self
            .calls
            .iter()
            .filter(|call| call.matches_expected_arguments(&arguments))
            .collect::<Vec<_>>();

        match matching_method_calls.len() {
            0 => Err(CallError::NoMatching(arguments, self)),
            1 => Ok(matching_method_calls.first().unwrap().call(arguments)),
            _ => Err(CallError::MoreThanOneMatching(
                arguments,
                matching_method_calls,
            )),
        }
    }
}

#[derive(Debug)]
enum CallError<'a, 'mock, A, R>
where
    A: ArgumentsMatcher<'mock> + 'mock,
{
    NoMatching(A::Arguments, &'a Method<'mock, A, R>),
    MoreThanOneMatching(A::Arguments, Vec<&'a MethodCall<'mock, A, R>>),
}

impl<'a, 'mock, A, R> Display for CallError<'a, 'mock, A, R>
where
    A: ArgumentsMatcher<'mock> + 'mock,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallError::NoMatching(arguments, method) => write!(
                f,
                r#"
The call {:?} was not expected.
The following calls were expected:
{}
"#,
                "<arguments>", // DebugArguments::<A>::new(arguments),
                DisplayCalls(&method.calls.iter().collect::<Vec<_>>())
            ),
            CallError::MoreThanOneMatching(arguments, calls) => write!(
                f,
                r#"
The call {:?} matches more than one expected call:
{}
"#,
                "<arguments>", // DebugArguments::<A>::new(arguments),
                DisplayCalls(&calls)
            ),
        }
    }
}

struct DisplayCalls<'a, 'mock, A, R>(&'a [&'a MethodCall<'mock, A, R>])
where
    A: ArgumentsMatcher<'mock> + 'mock;

impl<'a, 'mock, A, R> Display for DisplayCalls<'a, 'mock, A, R>
where
    A: ArgumentsMatcher<'mock> + 'mock,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for call in self.0 {
            write!(f, "{}\n", call)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::matcher::IntoArgumentMatcher;

    #[test]
    fn call_errors_if_more_than_one_call_matches() {
        let mut method = Method::new("test");

        method
            .add_expected_call(("foo".into_argument_matcher(),))
            .returns(String::from("bar"));

        method
            .add_expected_call(("foo".into_argument_matcher(),))
            .returns(String::from("baz"));

        match method.call(("foo",)) {
            Err(CallError::MoreThanOneMatching(args, method_calls)) => {
                assert_eq!(args, ("foo",));
                assert_eq!(2, method_calls.len());
            }
            _ => panic!("unexpected result from method call"),
        }
    }

    #[test]
    fn call_errors_if_no_calls_match() {
        let mut method = Method::new("test");

        method
            .add_expected_call(("foo".into_argument_matcher(),))
            .returns(String::from("bar"));

        match method.call(("bar",)) {
            Err(CallError::NoMatching(args, _)) => {
                assert_eq!(args, ("bar",));
            }
            _ => panic!("unexpected result from method call"),
        }
    }

    #[test]
    fn call_calls_matching_method_call() {
        let mut method = Method::new("test");

        method
            .add_expected_call(("foo".into_argument_matcher(),))
            .returns(String::from("bar"));

        assert_eq!(String::from("bar"), method.call(("foo",)).unwrap());
    }
}
