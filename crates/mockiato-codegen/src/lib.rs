//! Codegen for `mockiato`. Do not use this crate directly.

#![recursion_limit = "128"]
#![cfg_attr(rustc_is_nightly, feature(proc_macro_diagnostic))]
#![warn(clippy::dbg_macro, clippy::unimplemented, unreachable_pub)]
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
mod diagnostic;
mod emit_diagnostics;
mod generate;
mod mockable;
mod parse;
mod result;
mod syn_ext;

use self::mockable::Mockable;
use crate::emit_diagnostics::emit_diagnostics;
use proc_macro::TokenStream as ProcMacroTokenStream;
use syn::{parse_macro_input, AttributeArgs, Item};

#[doc(hidden)]
#[proc_macro_attribute]
pub fn mockable(args: ProcMacroTokenStream, input: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let original_input = input.clone();

    let attr = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as Item);

    let mockable = Mockable::new();

    match mockable.expand(attr, item) {
        Ok(output) => ProcMacroTokenStream::from(output),
        Err(error) => {
            let mut output = original_input;

            let diagnostics_output = emit_diagnostics(error);
            output.extend(ProcMacroTokenStream::from(diagnostics_output));

            output
        }
    }
}
