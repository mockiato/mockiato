use std::fmt::Debug;

trait Greeter: Debug {
    fn say_hello(&self, name: &str) -> String;

    fn print_hello(&self, name: &str);
}

#[derive(Debug)]
struct GreeterMock {
    say_hello: mockiato::internal::Method<self::greeter_mock::SayHelloArgumentsMatcher, String>,
    print_hello: mockiato::internal::Method<self::greeter_mock::PrintHelloArgumentsMatcher, ()>,
}

impl GreeterMock {
    fn new() -> Self {
        GreeterMock {
            say_hello: mockiato::internal::Method::new("GreeterMock::say_hello"),
            print_hello: mockiato::internal::Method::new("GreeterMock::print_hello"),
        }
    }

    // #[must_use] should only be here, if the function has a return
    #[must_use]
    fn expect_say_hello<A0>(
        &mut self,
        name: A0,
    ) -> mockiato::internal::MethodCallBuilder<
        '_,
        self::greeter_mock::SayHelloArgumentsMatcher,
        String,
    >
    where
        A0: for<'a> mockiato::internal::ArgumentMatcher<&'a str> + 'static,
    {
        self.say_hello
            .add_expected_call(self::greeter_mock::SayHelloArgumentsMatcher {
                name: Box::new(name),
            })
    }

    fn expect_print_hello<A0>(
        &mut self,
        name: A0,
    ) -> mockiato::internal::MethodCallBuilder<'_, self::greeter_mock::PrintHelloArgumentsMatcher, ()>
    where
        A0: for<'a> mockiato::internal::ArgumentMatcher<&'a str> + 'static,
    {
        self.print_hello
            .add_expected_call(self::greeter_mock::PrintHelloArgumentsMatcher {
                name: Box::new(name),
            })
    }
}

impl Drop for GreeterMock {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.say_hello.verify_unwrap();
            self.print_hello.verify_unwrap();
        }
    }
}

impl Greeter for GreeterMock {
    fn say_hello(&self, name: &str) -> String {
        self.say_hello
            .call_unwrap(self::greeter_mock::SayHelloArguments { name })
    }

    fn print_hello(&self, name: &str) {
        self.print_hello
            .call_unwrap(self::greeter_mock::PrintHelloArguments { name })
    }
}

mod greeter_mock {
    use std::fmt::{self, Debug};

    pub(super) struct SayHelloArguments<'args> {
        pub(super) name: &'args str,
    }

    impl<'args> Debug for SayHelloArguments<'args> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("")
                .field(&mockiato::internal::MaybeDebugWrapper(&self.name))
                .finish()
        }
    }

    impl<'args> mockiato::internal::Arguments for SayHelloArguments<'args> {}

    pub(super) struct SayHelloArgumentsMatcher {
        pub(super) name: Box<dyn for<'a> mockiato::internal::ArgumentMatcher<&'a str>>,
    }

    impl Debug for SayHelloArgumentsMatcher {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("say_hello")
                .field(&mockiato::internal::MaybeDebugExtWrapper(&self.name))
                .finish()
        }
    }

    impl<'args> mockiato::internal::ArgumentsMatcher<'args> for SayHelloArgumentsMatcher {
        type Arguments = SayHelloArguments<'args>;

        fn matches_arguments(&self, args: &Self::Arguments) -> bool {
            self.name.matches_argument(&args.name)
        }
    }

    pub(super) struct PrintHelloArguments<'args> {
        pub(super) name: &'args str,
    }

    impl<'args> Debug for PrintHelloArguments<'args> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("")
                .field(&mockiato::internal::MaybeDebugWrapper(&self.name))
                .finish()
        }
    }

    impl<'args> mockiato::internal::Arguments for PrintHelloArguments<'args> {}

    pub(super) struct PrintHelloArgumentsMatcher {
        pub(super) name: Box<dyn for<'a> mockiato::internal::ArgumentMatcher<&'a str>>,
    }

    impl Debug for PrintHelloArgumentsMatcher {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("print_hello")
                .field(&mockiato::internal::MaybeDebugExtWrapper(&self.name))
                .finish()
        }
    }

    impl<'args> mockiato::internal::ArgumentsMatcher<'args> for PrintHelloArgumentsMatcher {
        type Arguments = PrintHelloArguments<'args>;

        fn matches_arguments(&self, args: &Self::Arguments) -> bool {
            self.name.matches_argument(&args.name)
        }
    }
}

#[test]
fn hand_generated_mock_works() {
    let mut mock = GreeterMock::new();

    mock.expect_say_hello("foo")
        .returns(String::from("Hello foo"))
        .times(1);

    mock.expect_say_hello("bar")
        .returns(String::default())
        .times(4);

    mock.expect_say_hello("baz").panics_with_message("foo");

    mock.expect_print_hello("foo").times(..=8);

    mock.say_hello("foo");

    mock.say_hello("bar");
    mock.say_hello("bar");
    mock.say_hello("bar");
    mock.say_hello("bar");

    mock.print_hello("foo");
}
