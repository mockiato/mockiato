use mockiato::mockable;
use std::fmt::Display;

#[mockable(static_references)]
trait Greeter<T>
where
    T: Display,
{
    fn greet(&self, name: T) -> String;
}

#[test]
fn test_static_references_works_together_with_generic_types() {
    let name = "Name";
    let expected_greeting = String::from("Hello Name");

    let greeter: Box<dyn Greeter<&str>> = {
        let mut greeter = GreeterMock::new();
        greeter
            .expect_greet(|arg| arg.partial_eq(name))
            .returns(expected_greeting.clone());
        Box::new(greeter)
    };

    assert_eq!(expected_greeting, greeter.greet(name));
}
