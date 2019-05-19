use super::ReturnValueGenerator;
use crate::internal::fmt::DisplayOption;
use crate::internal::ArgumentsMatcher;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Panic(pub(crate) Option<&'static str>);

impl Display for Panic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "panic!({})", DisplayOption(self.0.as_ref()))
    }
}

impl<A, R> ReturnValueGenerator<A, R> for Panic
where
    A: for<'args> ArgumentsMatcher<'args>,
{
    fn generate_return_value(&self, _: <A as ArgumentsMatcher<'_>>::Arguments) -> R {
        match self.0 {
            Some(message) => panic!(message),
            None => panic!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::internal::arguments::ArgumentsMock;
    use crate::internal::matcher::ArgumentsMatcherMock;

    #[test]
    #[should_panic(expected = "<panic message>")]
    fn test_panic_panicks() {
        let panic = Panic(Some("<panic message>"));

        ReturnValueGenerator::<ArgumentsMatcherMock, ()>::generate_return_value(
            &panic,
            ArgumentsMock,
        );
    }
}
