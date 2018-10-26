use std::fmt::Debug;

pub trait Arguments: Debug {}

#[cfg(test)]
pub(crate) use self::mock::*;

#[cfg(test)]
mod mock {
    use super::Arguments;

    #[derive(Debug)]
    pub(crate) struct ArgumentsMock;

    impl Arguments for ArgumentsMock {}
}
