use mockiato::mockable;

#[mockable]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

#[test]
fn test() {
    let name = String::from("Peter Parker");
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet(|f| f.partial_eq(&name))
        .returns(String::from("Hello Peter Parker"));

    assert_eq!(
        String::from("Hello Peter Parker"),
        greeter.greet("Peter Parker")
    );
}
