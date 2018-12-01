mod greeter {
    use mockiato_codegen::mockable;

    #[derive(Eq, PartialEq)]
    pub(super) struct Name {
        pub(super) name: &'static str,
    }

    #[mockable]
    pub(super) trait Greeter {
        fn greet(&self, name: Name) -> String;
    }
}

fn main() {
    let mut mock = greeter::GreeterMock::new();

    mock.expect_greet(greeter::Name { name: "Peter" })
        .returns(String::from("Hello Peter"));
}
