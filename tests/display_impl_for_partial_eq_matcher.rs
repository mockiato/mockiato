use mockiato::{mockable, partial_eq};

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[mockable]
trait Greeter {
    fn greet(&self, person: Person) -> String;

    fn greet_ref(&self, person: &Person) -> String;
}

#[test]
fn partial_eq_matcher_has_debug_output() {
    let matcher = partial_eq(Person {
        name: "Name".into(),
        age: 30,
    });

    assert_eq!(
        r#"Person { name: "Name", age: 30 }"#,
        format!("{}", matcher)
    );
}

#[test]
#[should_panic(
    expected = "The expected calls for GreeterMock::greet were not satisified.
greet(Person { name: \"Name\", age: 30 }) -> \"Hello Name\" exactly 2 times, was called 0 times"
)]
fn partial_eq_matcher_has_debug_output_when_printed_as_expected_call() {
    let mut greeter = GreeterMock::new();
    greeter
        .expect_greet(partial_eq(Person {
            name: "Name".into(),
            age: 30,
        }))
        .times(2)
        .returns(String::from("Hello Name"));
}

#[test]
#[should_panic(
    expected = "The expected calls for GreeterMock::greet_ref were not satisified.
greet_ref(Person { name: \"Name\", age: 30 }) -> \"Hello Name\" exactly 2 times, was called 0 times"
)]
fn partial_eq_matcher_has_debug_output_for_reference_when_printed_as_expected_call_with() {
    let person = Person {
        name: "Name".into(),
        age: 30,
    };
    let mut greeter = GreeterMock::new();
    greeter
        .expect_greet_ref(partial_eq(&person))
        .times(2)
        .returns(String::from("Hello Name"));
}
