pub(crate) use self::mockable_attr_parser_impl::*;
use crate::result::Result;
use std::fmt::Debug;
use syn::{AttributeArgs, Ident, Path};

mod mockable_attr_parser_impl;

/// The `#[mockable]` attribute, which is placed on a trait.
#[cfg_attr(feature = "debug-impls", derive(Debug))]
#[derive(Default)]
pub(crate) struct MockableAttr {
    /// Customizes the name of the generated mock struct.
    /// Example usage: `#[name = "FooMock"]`
    pub(crate) name: Option<Ident>,
    /// Enforces that only static lifetimes are used within the mock.
    /// Example usage: `#[mockable(static_references)]`.
    pub(crate) force_static_lifetimes: bool,
    /// Enables mocking of a remote trait.
    /// Example usage: `#[mockable(remote = "io::Write")]`
    pub(crate) remote_trait_path: Option<RemoteTraitPath>,
}

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) enum RemoteTraitPath {
    /// Corresponds to the `remote` parameter without a value:  
    /// `#[mockable(remote)]`
    SameAsLocalIdent,
    /// Corresponds to the `remote` parameter with a [`Path`] as value:  
    /// `#[mockable(remote = "std::io::Write")]`
    Path(Path),
}

pub(crate) trait MockableAttrParser: Debug {
    fn parse(&self, args: AttributeArgs) -> Result<MockableAttr>;
}
