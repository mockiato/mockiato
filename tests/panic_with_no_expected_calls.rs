use mockiato::mockable;

#[mockable]
trait Greeter {
    fn greet(&self, name: &str) -> String;
    fn greet_two_people(&self, first_name: &str, second_name: &str) -> String;
    fn greet_unknown_person(&self) -> String;
}

#[cfg(rustc_is_nightly)]
#[test]
#[should_panic(
    expected = "The call GreeterMock::greet_unknown_person() was not expected.\nNo calls to \
                GreeterMock::greet_unknown_person were expected."
)]
fn panics_with_no_expected_calls_with_no_arguments() {
    let greeter = GreeterMock::new();

    greeter.greet_unknown_person();
}

#[cfg(rustc_is_nightly)]
#[test]
#[should_panic(
    expected = "The call GreeterMock::greet(\"John\") was not expected.\nNo calls to \
                GreeterMock::greet were expected."
)]
fn panics_with_no_expected_calls_with_one_argument() {
    let greeter = GreeterMock::new();

    greeter.greet("John");
}

#[cfg(rustc_is_nightly)]
#[test]
#[should_panic(
    expected = "The call GreeterMock::greet_two_people(\"John\", \"Adam\") was not expected.\nNo \
                calls to GreeterMock::greet_two_people were expected."
)]
fn panics_with_no_expected_calls_with_two_argument() {
    let greeter = GreeterMock::new();

    greeter.greet_two_people("John", "Adam");
}
