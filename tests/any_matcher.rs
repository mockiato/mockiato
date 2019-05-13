use mockiato::{mockable, Argument};

#[mockable]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[test]
fn cloneable_mocks_work() {
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet(Argument::any)
        .times(2)
        .returns(String::from("Hello Tom"));

    assert_eq!("Hello Tom", greeter.greet("Tom"));
    assert_eq!("Hello Tom", greeter.greet("Tum"));
}
