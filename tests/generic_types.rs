use mockiato::mockable;
use std::fmt::{self, Display};

trait Foo {
    type Output;
}

#[mockable]
trait Greeter<T, V>
where
    T: Display,
    V: Foo,
    V::Output: Display,
{
    fn generic_param_as_argument(&self, name: T) -> String;

    fn associated_type(&self, name: V::Output) -> String;

    fn generic_param_wrapped_in_container(&self, names: Vec<T>) -> String;

    fn generic_param_as_return_value(&self, name: String) -> T;
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
    let mut mock: GreeterMock<Name, String> = GreeterMock::new();
    const EXPECTED_NUMBER_OF_CALLS: u64 = 2;

    mock.expect_generic_param_as_argument(|arg| arg.partial_eq(Name::new("Foo")))
        .times(EXPECTED_NUMBER_OF_CALLS)
        .returns(String::from("Hello Foo"));

    for _ in 0..EXPECTED_NUMBER_OF_CALLS {
        assert_eq!(
            "Hello Foo",
            mock.generic_param_as_argument(Name::new("Foo"))
        );
    }
}
