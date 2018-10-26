use crate::internal::matcher::ArgumentsMatcher;
use crate::internal::method_call::{MethodCall, MethodCallBuilder};
use std::fmt::{self, Display};
use std::thread::panicking;

#[derive(Debug)]
pub struct Method<A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    name: &'static str,
    calls: Vec<MethodCall<A, R>>,
}

impl<A, R> Method<A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: Default::default(),
        }
    }

    pub fn add_expected_call(&mut self, matcher: A) -> MethodCallBuilder<'_, A, R> {
        let call = MethodCall::new(matcher);

        self.calls.push(call);

        MethodCallBuilder::new(self.calls.last_mut().unwrap())
    }

    pub fn call_unwrap<'a>(&'a self, arguments: <A as ArgumentsMatcher<'a>>::Arguments) -> R {
        match self.call(arguments) {
            Ok(return_value) => return_value,
            Err(err) => panic!("{}", err),
        }
    }

    pub fn verify_unwrap(&self) {
        if let Err(err) = self.verify() {
            panic!("{}", err);
        }
    }

    fn call<'a>(
        &'a self,
        arguments: <A as ArgumentsMatcher<'a>>::Arguments,
    ) -> Result<R, CallError<'a, A, R>> {
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

    fn verify(&self) -> Result<(), VerificationError<'_, A, R>> {
        if self
            .calls
            .iter()
            .any(|method_call| !method_call.was_called_expected_number_of_times())
        {
            return Err(VerificationError { method: &self });
        }

        Ok(())
    }
}

#[derive(Debug)]
enum CallError<'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    NoMatching(<A as ArgumentsMatcher<'a>>::Arguments, &'a Method<A, R>),
    MoreThanOneMatching(
        <A as ArgumentsMatcher<'a>>::Arguments,
        Vec<&'a MethodCall<A, R>>,
    ),
}

impl<'a, A, R> Display for CallError<'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
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
                arguments,
                DisplayCalls(&method.calls.iter().collect::<Vec<_>>())
            ),
            CallError::MoreThanOneMatching(arguments, calls) => write!(
                f,
                r#"
The call {:?} matches more than one expected call:
{}
"#,
                arguments,
                DisplayCalls(&calls)
            ),
        }
    }
}

#[derive(Debug)]
struct VerificationError<'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    method: &'a Method<A, R>,
}

impl<'a, A, R> Display for VerificationError<'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "The expected calls for {} were not satisified.",
            self.method.name
        )?;

        for call in &self.method.calls {
            writeln!(f, "{}", call)?;
        }

        Ok(())
    }
}

struct DisplayCalls<'a, A, R>(&'a [&'a MethodCall<A, R>]);

impl<'a, A, R> Display for DisplayCalls<'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for call in self.0 {
            writeln!(f, "{}", call)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::arguments::ArgumentsMock;
    use crate::internal::matcher::ArgumentsMatcherMock;

    #[test]
    fn call_errors_if_more_than_one_call_matches() {
        let mut method = Method::<_, ()>::new("test");

        method.add_expected_call(ArgumentsMatcherMock::new(Some(true)));

        method.add_expected_call(ArgumentsMatcherMock::new(Some(true)));

        match method.call(ArgumentsMock) {
            Err(CallError::MoreThanOneMatching(args, method_calls)) => {
                assert_eq!(2, method_calls.len());
            }
            _ => panic!("unexpected result from method call"),
        }
    }

    #[test]
    fn call_errors_if_no_calls_match() {
        let mut method = Method::<_, ()>::new("test");

        method
            .add_expected_call(ArgumentsMatcherMock::new(Some(false)))
            .returns(());

        match method.call(ArgumentsMock) {
            Err(CallError::NoMatching(..)) => {}
            _ => panic!("unexpected result from method call"),
        }
    }

    #[test]
    fn call_calls_matching_method_call() {
        let mut method = Method::<_, String>::new("test");

        method
            .add_expected_call(ArgumentsMatcherMock::new(Some(true)))
            .returns(String::from("bar"));

        assert_eq!(String::from("bar"), method.call(ArgumentsMock).unwrap());
    }

    #[test]
    fn verify_is_ok_if_expectations_are_met() {
        let mut method = Method::<_, String>::new("test");

        method
            .add_expected_call(ArgumentsMatcherMock::new(Some(true)))
            .returns(Default::default())
            .times(1);

        method.call(ArgumentsMock).unwrap();

        assert!(method.verify().is_ok());
    }

    #[test]
    fn verify_errors_if_expectations_not_met() {
        let mut method = Method::<_, String>::new("test");

        method
            .add_expected_call(ArgumentsMatcherMock::new(None))
            .returns(Default::default())
            .times(2);

        assert!(method.verify().is_err());
    }

    #[test]
    fn verify_is_ok_if_expectations_are_empty() {
        let method = Method::<ArgumentsMatcherMock, String>::new("test");

        assert!(method.verify().is_ok());
    }
}
