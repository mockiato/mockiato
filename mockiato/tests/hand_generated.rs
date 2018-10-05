use std::fmt::Display;

trait Greeter<D>: Display
where
    D: Display,
{
    fn say_hello(&self, name: D) -> String;

    fn print_hello(&self, name: D);
}

struct GreeterMock<'mock, D>
where
    D: Display,
{
    say_hello_calls: mockiato::Calls<'mock, (D,), String>,
    print_hello_calls: mockiato::Calls<'mock, (D,), ()>,
}

impl<'mock, D> GreeterMock<'mock, D>
where
    D: Display,
{
    fn new() -> Self {
        GreeterMock {
            say_hello_calls: mockiato::Calls::new("say_hello"),
            print_hello_calls: mockiato::Calls::new("print_hello"),
        }
    }

    // #[must_use] should only be here, if the function has a return
    #[must_use]
    fn expect_say_hello<A0>(&self, name: A0) -> mockiato::CallBuilder<'mock, (D,), String>
    where
        A0: mockiato::IntoArgumentMatcher<'mock, D>,
    {
        let matchers = (name.into_argument_matcher(),);

        self.say_hello_calls.expect(matchers)
    }

    fn expect_print_hello<A0>(&self, name: A0) -> mockiato::CallBuilder<'mock, (D,), ()>
    where
        A0: mockiato::IntoArgumentMatcher<'mock, D>,
    {
        let matchers = (name.into_argument_matcher(),);

        self.print_hello_calls.expect(matchers)
    }
}

#[test]
fn test_hand_generated_mock_works() {
    let mock = GreeterMock::new();

    mock.expect_say_hello("foo")
        .will_return(String::from("Hello foo"));

    mock.expect_say_hello("bar")
        .will_return(Default::default())
        .times(4);

    mock.expect_print_hello("foo");
}
