use std::marker::PhantomData;

/// A factory for creating argument matchers
#[derive(Debug)]
pub struct ArgumentMatcherFactory(PhantomData<()>);

impl ArgumentMatcherFactory {
    #[doc(hidden)]
    pub fn internal_new() -> Self {
        Self(PhantomData)
    }
}
