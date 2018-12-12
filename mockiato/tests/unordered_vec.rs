use mockiato::{mockable, unordered_vec_eq};

#[mockable]
trait Greeter {
    fn greet(&self, names: &[&str]) -> String;
}

#[test]
fn unordered_slice_matcher_works() {
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet(unordered_vec_eq(vec!["Tom", "Peter", "Hans"]))
        .returns(String::from("Hello everyone"));

    assert_eq!("Hello everyone", greeter.greet(&["Peter", "Hans", "Tom"]));
}
