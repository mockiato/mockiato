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
#![warn(clippy::dbg_macro, clippy::unimplemented)]
#![deny(
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    clippy::doc_markdown,
    clippy::default_trait_access,
    clippy::enum_glob_use,
    clippy::needless_borrow,
    clippy::large_digit_groups,
    clippy::explicit_into_iter_loop
)]

extern crate proc_macro;

mod constant;
mod generate;
mod mockable;
mod parse;
mod result;
mod spanned;

use self::mockable::Mockable;
pub(crate) use self::result::*;
use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Item};

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
