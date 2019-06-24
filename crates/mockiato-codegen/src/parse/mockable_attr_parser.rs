pub(crate) use self::mockable_attr_parser_impl::*;
use crate::result::Result;
use syn::{AttributeArgs, Ident};

mod mockable_attr_parser_impl;

/// The `#[mockable]` attribute, which is placed on a trait.
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MockableAttr {
    /// Customizes the name of the generated mock struct.
    /// Example: `#[name = "FooMock"]`
    pub(crate) name: Option<Ident>,
    /// The static sub-attribute. Example: `#[mockable(static_references)]`.
    /// Enforces that only static lifetimes are used within the mock.
    pub(crate) enforce_static_references: bool,
}

pub(crate) trait MockableAttrParser {
    fn parse(&self, args: AttributeArgs) -> Result<MockableAttr>;
}
