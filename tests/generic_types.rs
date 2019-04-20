use mockiato::mockable;
use std::fmt::{self, Display};

#[mockable]
trait Greeter<T>
where
    T: Display,
{
    fn greet(&self, name: T) -> String;
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
