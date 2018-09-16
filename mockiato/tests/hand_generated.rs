use std::fmt::{self, Display};

trait Greeter<D>: fmt::Debug
where
    D: Display,
{
    fn say_hello(&self, name: D) -> String;
}

#[derive(Debug, Clone)]
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
    pub fn new() -> Self {
        Self {
            say_hello_calls: mockiato::Calls::new("say_hello"),
        }
    }

    pub fn expect_say_hello<F>(
        &mut self,
        matcher: Box<dyn mockiato::CallMatcher<(D,)> + 'mock>,
        with_fn: F,
    ) where
        F: FnOnce(&mut mockiato::Call<'mock, (D,), String>),
    {
        let mut call = mockiato::Call::new("say_hello", matcher);

        with_fn(&mut call);

        self.say_hello_calls.add(call);
    }
}

impl<'mock, D> Drop for GreeterMock<'mock, D>
where
    D: Display,
{
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.say_hello_calls.verify();
        }
    }
}

impl<'mock, D> Greeter<D> for GreeterMock<'mock, D>
where
    D: Display + std::fmt::Debug,
{
    fn say_hello(&self, name: D) -> String {
        self.say_hello_calls.call((name,))
    }
}

#[test]
fn test() {
    use mockiato::args;

    let mock: GreeterMock<&str> = {
        let mut mock = GreeterMock::new();

        mock.expect_say_hello(args(("foo",)), |call| {
            call.will_return(String::from("Hello foo")).times(1);
        });

        mock.expect_say_hello(args(("bar",)), |call| {
            call.will_return(String::from("Hello bar")).times(1);
        });

        mock
    };

    assert_eq!("Hello bar", mock.say_hello("bar"));
    assert_eq!("Hello foo", mock.say_hello("foo"));
    assert_eq!("Hello foo", mock.say_hello("foo"));
}
