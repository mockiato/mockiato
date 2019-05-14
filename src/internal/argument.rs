use std::marker::PhantomData;

/// A factory for creating argument matchers
#[derive(Debug)]
pub struct Argument(PhantomData<()>);

impl Argument {
    #[doc(hidden)]
    pub fn internal_new() -> Self {
        Self(PhantomData)
    }
}
