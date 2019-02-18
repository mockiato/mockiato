use mockiato::{mockable, partial_eq};

#[mockable]
trait Greeter {
    fn greeting(&self) -> &str;
}

#[test]
fn test() {
    let greeting = String::from("Foo");
    let mut greeter = GreeterMock::new();

    greeter.expect_greeting().returns(&greeting);

    assert_eq!("Foo", greeter.greeting())
}
