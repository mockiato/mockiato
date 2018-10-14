use std::fmt::{Debug, Display};

trait Greeter /*: Debug*/ {
    fn say_hello(&self, name: &str) -> String;

    fn print_hello(&self, name: &str);
}

// #[derive(Debug)]
struct GreeterMock<'mock> {
    say_hello: mockiato::internal::Method<'mock, ((&'mock str),), String>,
    print_hello: mockiato::internal::Method<'mock, ((&'mock str),), ()>,
}

impl<'mock> GreeterMock<'mock> {
    fn new() -> Self {
        GreeterMock {
            say_hello: mockiato::internal::Method::new("GreeterMock::say_hello"),
            print_hello: mockiato::internal::Method::new("GreeterMock::print_hello"),
        }
    }

    // #[must_use] should only be here, if the function has a return
    #[must_use]
    fn expect_say_hello<A0>(
        &mut self,
        name: A0,
    ) -> mockiato::internal::MethodCallBuilder<'_, 'mock, (&'mock str,), String>
    where
        A0: mockiato::internal::IntoArgumentMatcher<'mock, &'mock str>,
    {
        let matchers = (name.into_argument_matcher(),);

        self.say_hello.add_expected_call(matchers)
    }

    fn expect_print_hello<A0>(
        &mut self,
        name: A0,
    ) -> mockiato::internal::MethodCallBuilder<'_, 'mock, (&'mock str,), ()>
    where
        A0: mockiato::internal::IntoArgumentMatcher<'mock, &'mock str>,
    {
        let matchers = (name.into_argument_matcher(),);

        self.print_hello.add_expected_call(matchers)
    }
}

impl<'mock> Greeter for GreeterMock<'mock> {
    fn say_hello(&self, name: &str) -> String {
        self.say_hello.call_unwrap((name,))
    }

    fn print_hello(&self, name: &str) {
        self.print_hello.call_unwrap((name,))
    }
}

#[test]
fn hand_generated_mock_works() {
    let mut mock = GreeterMock::new();

    mock.expect_say_hello("foo")
        .returns(String::from("Hello foo"));

    mock.expect_say_hello("bar")
        .returns(String::default())
        .times(4);

    mock.expect_say_hello("baz").panics_with_message("foo");

    mock.expect_print_hello("foo").times(..=8);
}
