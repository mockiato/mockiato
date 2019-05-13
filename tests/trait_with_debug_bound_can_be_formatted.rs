use mockiato::mockable;
use std::fmt::Debug;

#[mockable]
trait Greeter<T, U>: Debug {
    fn greet(&self, name: T) -> U;
}

#[derive(Eq, PartialEq)]
struct Name;

#[derive(Clone)]
struct Greeting;

#[test]
fn trait_with_debug_bound_can_be_formatted() {
    let mut greeter: GreeterMock<'_, Name, Greeting> = GreeterMock::new();

    let mut builder = greeter.expect_greet(|a| a.partial_eq(Name));
    builder.times(..).returns(Greeting);

    let _assert_builder_can_be_debug_formatted = format!("{:?}", builder);
}
