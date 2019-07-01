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

mod code_generator;
mod code_generator_impl;
mod constant;
mod controller_impl;
mod diagnostic;
mod emit_diagnostics;
mod parse;
mod result;
mod syn_ext;

use crate::code_generator::CodeGenerator;
use crate::code_generator_impl::{
    ArgumentsMatcherGenerator, ArgumentsMatcherGeneratorImpl, CodeGeneratorImpl,
};
use crate::controller_impl::ControllerImpl;
use crate::emit_diagnostics::emit_diagnostics;
use crate::parse::mockable_attr_parser::{MockableAttrParser, MockableAttrParserImpl};
use crate::result::Result;
use proc_macro::TokenStream as ProcMacroTokenStream;
use proc_macro2::TokenStream;
use syn::{parse_macro_input, AttributeArgs, Item};
use wonderbox::Container;

pub(crate) trait Controller {
    fn expand_mockable_trait(&self, attr: AttributeArgs, item: Item) -> Result<TokenStream>;
}

#[doc(hidden)]
#[proc_macro_attribute]
pub fn mockable(args: ProcMacroTokenStream, input: ProcMacroTokenStream) -> ProcMacroTokenStream {
    let original_input = input.clone();

    let attr = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as Item);

    let container = build_composition_root();
    let controller: Box<dyn Controller> = container.resolve();
    match controller.expand_mockable_trait(attr, item) {
        Ok(output) => ProcMacroTokenStream::from(output),
        Err(error) => {
            let mut output = original_input;

            let diagnostics_output = emit_diagnostics(error);
            output.extend(ProcMacroTokenStream::from(diagnostics_output));

            output
        }
    }
}

macro_rules! container {
    ($($type:ty = $container:ident => $factory:expr,)+) => {{
        let mut container = Container::new();
        $(
            container.register(|$container| $factory as $type);
        )+
        container
    }}
}

fn build_composition_root() -> Container {
    container!(
        Box<dyn MockableAttrParser> = _c => Box::new(MockableAttrParserImpl::new()),
        Box<dyn ArgumentsMatcherGenerator> = _c => Box::new(ArgumentsMatcherGeneratorImpl::new()),
        Box<dyn CodeGenerator> = c => Box::new(CodeGeneratorImpl::new(c.resolve())),
        Box<dyn Controller> = c => Box::new(ControllerImpl::new(c.resolve(), c.resolve())),
    )
}
