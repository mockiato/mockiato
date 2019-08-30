use std::fmt::{Debug, Display};

#[allow(missing_docs)]
pub trait Arguments: Display + Debug {}

#[cfg(test)]
pub(crate) use self::mock::*;

#[cfg(test)]
mod mock {
    use super::Arguments;
    use std::fmt;

    #[derive(Debug)]
    pub(crate) struct ArgumentsMock;

    impl Arguments for ArgumentsMock {}

    impl std::fmt::Display for ArgumentsMock {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "mock")
        }
    }
}
