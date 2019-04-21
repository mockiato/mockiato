use mockiato::mockable;
use std::fmt::{self, Display, Debug};

trait Foo {
    type Output;
}

#[mockable]
trait Greeter<T, U, V>
where
    T: Display,
    U: Debug,
    V: Foo,
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

impl Foo for String {
    type Output = String;
}

#[test]
fn trait_with_generic_type_argument_can_be_mocked() {
    let mock: GreeterMock<Name, (), String> = GreeterMock::new();
}
