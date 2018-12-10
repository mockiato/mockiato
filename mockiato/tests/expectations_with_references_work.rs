use mockiato::{mockable, partial_eq};

#[mockable]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[test]
fn cloneable_mocks_work() {
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet(partial_eq("Tom"))
        .times(2)
        .returns(String::from("Hello Tom"));

    greeter
        .expect_greet(partial_eq(String::from("Peter")))
        .returns(String::from("Hello Peter"));

    assert_eq!("Hello Tom", greeter.greet("Tom"));
    assert_eq!("Hello Tom", greeter.greet("Tom"));

    assert_eq!("Hello Peter", greeter.greet("Peter"));
}
