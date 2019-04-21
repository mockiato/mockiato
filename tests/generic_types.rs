use mockiato::{mockable, partial_eq};
use std::fmt::{self, Debug, Display};

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

#[derive(PartialEq, Eq)]
struct Name {
    name: String,
}

impl Name {
    fn new<S>(name: S) -> Self
    where
        S: Into<String>,
    {
        Self { name: name.into() }
    }
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
    let mut mock: GreeterMock<Name, (), String> = GreeterMock::new();

    mock.expect_greet(partial_eq(Name::new("Foo")))
        .times(2)
        .returns(String::from("Hello Foo"));

    for _ in 0..2 {
        assert_eq!("Hello Foo", mock.greet(Name::new("Foo")));
    }
}
