use mockiato::mockable;

#[mockable]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[test]
#[should_panic(
    expected = "The call GreeterMock::greet(\"John\") was not expected.\nNo calls to GreeterMock::greet were expected."
)]
fn panics_with_no_expected_calls() {
    let greeter = GreeterMock::new();

    greeter.greet("John");
}
