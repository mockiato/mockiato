use crate::internal::matcher::ArgumentsMatcher;
use crate::internal::method_call::{MethodCall, MethodCallBuilder};
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Method<'mock, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    name: &'static str,
    calls: Vec<MethodCall<'mock, A, R>>,
}

impl<'mock, A, R> Clone for Method<'mock, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            calls: self.calls.clone(),
        }
    }
}

impl<'mock, A, R> Method<'mock, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            calls: Default::default(),
        }
    }

    pub fn add_expected_call(&mut self, matcher: A) -> MethodCallBuilder<'mock, '_, A, R> {
        let call = MethodCall::new(matcher);

        self.calls.push(call);

        MethodCallBuilder::new(self.calls.last_mut().unwrap())
    }

    pub fn call_unwrap<'a>(&'a self, arguments: <A as ArgumentsMatcher<'a>>::Arguments) -> R {
        self.call(arguments)
            .unwrap_or_else(|err| panic!("\n\n{}\n", err))
    }

    pub fn verify_unwrap(&self) {
        self.verify().unwrap_or_else(|err| panic!("{}", err))
    }

    fn call<'a>(
        &'a self,
        arguments: <A as ArgumentsMatcher<'a>>::Arguments,
    ) -> Result<R, CallError<'mock, 'a, A, R>> {
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

    fn verify(&self) -> Result<(), VerificationError<'mock, '_, A, R>> {
        if self
            .calls
            .iter()
            .any(|method_call| !method_call.was_called_expected_number_of_times())
        {
            Err(VerificationError { method: &self })
        } else {
            Ok(())
        }
    }
}

#[derive(Debug)]
enum CallError<'mock, 'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    NoMatching(
        <A as ArgumentsMatcher<'a>>::Arguments,
        &'a Method<'mock, A, R>,
    ),
    MoreThanOneMatching(
        <A as ArgumentsMatcher<'a>>::Arguments,
        Vec<&'a MethodCall<'mock, A, R>>,
    ),
}

impl<'mock, 'a, A, R> Display for CallError<'mock, 'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CallError::NoMatching(arguments, method) => {
                writeln!(
                    f,
                    "The call {}{:?} was not expected.",
                    method.name, arguments
                )?;

                if method.calls.is_empty() {
                    writeln!(f, "No calls to {} were expected.", method.name)
                } else {
                    writeln!(
                        f,
                        "The following calls were expected:\n{}",
                        DisplayCalls(&method.calls.iter().collect::<Vec<_>>())
                    )
                }
            }
            CallError::MoreThanOneMatching(arguments, calls) => writeln!(
                f,
                "\nThe call {:?} matches more than one expected call:\n{}",
                arguments,
                DisplayCalls(&calls)
            ),
        }
    }
}

#[derive(Debug)]
struct VerificationError<'mock, 'a, A, R>
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    method: &'a Method<'mock, A, R>,
}

impl<'mock, 'a, A, R> Display for VerificationError<'mock, 'a, A, R>
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

struct DisplayCalls<'mock, 'a, A, R>(&'a [&'a MethodCall<'mock, A, R>]);

impl<'mock, 'a, A, R> Display for DisplayCalls<'mock, 'a, A, R>
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
            Err(CallError::MoreThanOneMatching(_, method_calls)) => {
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
