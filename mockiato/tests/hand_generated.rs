use std::fmt::Display;

trait Greeter<D>: Display
where
    D: Display,
{
    fn say_hello(&self, name: D) -> String;
}

struct GreeterMock<'mock, D>
where
    D: Display,
{
    say_hello_calls: mockiato::Calls<'mock, (D,), String>,
}

impl<'mock, D> GreeterMock<'mock, D>
where
    D: Display,
{
    fn new() -> Self {
        GreeterMock {
            say_hello_calls: mockiato::Calls::new("say_hello"),
        }
    }

    #[must_use]
    fn expect_say_hello<A0>(&self, name: A0) -> mockiato::CallBuilder<'mock, (D,), String>
    where
        A0: mockiato::ArgumentMatcher<D> + 'mock,
    {
        let matchers = (Box::new(name) as Box<dyn mockiato::ArgumentMatcher<D>>,);

        self.say_hello_calls.expect(matchers)
    }
}

#[test]
fn test_hand_generated_mock_works() {
    let mock = GreeterMock::new();

    mock.expect_say_hello("foo")
        .will_return(String::from("Hello foo"));
}
