#![feature(
    quote,
    extern_crate_item_prelude,
    proc_macro_diagnostic,
    proc_macro_span,
    proc_macro_hygiene,
    bind_by_move_pattern_guards
)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

#[macro_use]
extern crate syn;

mod constant;
mod mockable;
mod parse;

use self::mockable::Mockable;
use proc_macro::{Diagnostic, TokenStream};
use syn::{AttributeArgs, Item};

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub(crate) enum Error {
    Empty,
    Diagnostic(Diagnostic),
    MultipleDiagnostics(Vec<Diagnostic>),
}

impl Error {
    pub(crate) fn emit<F>(self, map_fn: F) -> Self
    where
        F: Fn(Diagnostic) -> Diagnostic,
    {
        match self {
            Error::Empty => {}
            Error::Diagnostic(diagnostic) => map_fn(diagnostic).emit(),
            Error::MultipleDiagnostics(diagnostics) => {
                diagnostics.into_iter().for_each(|d| map_fn(d).emit());
            }
        };

        Error::Empty
    }

    pub(crate) fn merge<I>(errors: I) -> Self
    where
        I: Iterator<Item = Error>,
    {
        let mut collected = Vec::new();

        errors.for_each(|err| match err {
            Error::Empty => {}
            Error::Diagnostic(diagnostic) => collected.push(diagnostic),
            Error::MultipleDiagnostics(mut diagnostics) => collected.append(&mut diagnostics),
        });

        Error::MultipleDiagnostics(collected)
    }
}

#[proc_macro_attribute]
pub fn mockable(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(input as Item);

    let mockable = Mockable::new();

    mockable.expand(attr, item)
}
