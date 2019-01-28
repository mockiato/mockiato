//! Codegen for `mockiato`. Do not use this crate directly.

#![feature(
    proc_macro_diagnostic,
    proc_macro_span,
    proc_macro_hygiene,
    bind_by_move_pattern_guards,
    decl_macro,
    box_syntax,
    box_patterns
)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

mod constant;
mod generate;
mod mockable;
mod parse;
mod result;
mod spanned;

use self::mockable::Mockable;
pub(crate) use self::result::*;
use proc_macro::TokenStream;
use syn::{AttributeArgs, Item};

/// Generates a mock struct from a trait.
///
/// # Examples
///
/// ```ignore
/// use mockiato::mockable;
/// use std::fmt::Display;
///
/// #[mockable]
/// trait Greeter {
///     fn greet(&self, name: &Display) -> String;
/// }
/// ```
#[proc_macro_attribute]
pub fn mockable(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as Item);

    let mockable = Mockable::new();

    mockable.expand(attr, item)
}
