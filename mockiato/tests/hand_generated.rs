use std::fmt::{Debug, Display};

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
}
