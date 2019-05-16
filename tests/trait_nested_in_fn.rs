use mockiato::mockable;

fn main() {
    #[mockable]
    trait Greeter {
        fn greet(&self, name: &str) -> String;
    }

    let _greeter_mock = GreeterMock::new();
}
