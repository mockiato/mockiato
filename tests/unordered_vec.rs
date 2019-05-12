use mockiato::mockable;

#[mockable]
trait Greeter {
    fn greet(&self, names: &[&str]) -> String;
    fn greet_with_mut_names<'a>(&'a self, names: &'a mut [&'a str]) -> String;
}

#[test]
fn unordered_slice_matcher_works() {
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet(|f| f.unordered_vec_eq(vec!["Tom", "Peter", "Hans"]))
        .returns(String::from("Hello everyone"));

    greeter
        .expect_greet_with_mut_names(|f| f.unordered_vec_eq(vec!["Heidi", "Jerry"]))
        .returns(String::from("Hello 👋"));

    assert_eq!("Hello everyone", greeter.greet(&["Peter", "Hans", "Tom"]));

    assert_eq!(
        "Hello 👋",
        greeter.greet_with_mut_names(&mut ["Jerry", "Heidi"])
    );
}
