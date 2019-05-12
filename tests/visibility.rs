use mockiato::mockable;

#[mockable]
pub trait PubGreeter {
    fn greet(&self);
}

mod greeter {
    use mockiato::mockable;

    #[derive(Eq, PartialEq)]
    pub(super) struct Name {
        pub(super) name: &'static str,
    }

    #[mockable]
    pub(super) trait Greeter {
        fn greet(&self, name: Name) -> String;
    }

    pub(crate) mod submod {
        use mockiato::mockable;

        #[mockable]
        pub(crate) trait PubCrateGreeter {
            fn greet(&self);
        }
    }
}

fn main() {
    let mut mock = greeter::GreeterMock::new();

    mock.expect_greet(|f| f.partial_eq(greeter::Name { name: "Peter" }))
        .returns(String::from("Hello Peter"));

    let _pub_crate_greeter = greeter::submod::PubCrateGreeterMock::new();
}
