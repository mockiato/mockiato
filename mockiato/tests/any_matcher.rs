use mockiato::{mockable, AnyMatcher};

#[mockable]
trait Greeter {
    fn say_hello(&self, name: &str) -> String;
}

#[test]
fn main() {
    let mut greeter = GreeterMock::new();

    greeter
        .expect_say_hello(AnyMatcher)
        .times(2)
        .returns(String::default());

    greeter.say_hello("Foo");
    greeter.say_hello("Bar");
}
