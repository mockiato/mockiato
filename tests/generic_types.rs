use mockiato::mockable;
use std::fmt::{self, Display};

trait Foo {
    type Output;
}

#[mockable]
trait Greeter<T, U, V>
where
    T: Display,
    U: Debug,
    V::Output: Display,
{
    fn greet(&self, name: T) -> String;

    fn greet_debug(&self, name: U) -> String;

    fn greet_foo(&self, name: V::Output) -> String;
}

struct Name {
    name: String,
}

impl Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.name)
    }
}

#[test]
fn trait_with_generic_type_argument_can_be_mocked() {
    let mock: GreeterMock<Name> = GreeterMock::new();
}
