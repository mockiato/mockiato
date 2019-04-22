//! Minimalistic mocking framework, ready for Rust 2018

#![feature(specialization)]
#![warn(missing_docs, clippy::dbg_macro, clippy::unimplemented)]
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

pub use mockiato_codegen::mockable;

pub use crate::internal::expected_calls::ExpectedCalls;
pub use crate::internal::matcher::any::any;
pub use crate::internal::matcher::nearly_eq::{nearly_eq, nearly_eq_with_accuracy};
pub use crate::internal::matcher::partial_eq::{partial_eq, partial_eq_owned};
pub use crate::internal::matcher::unordered_vec_eq::unordered_vec_eq;
pub use crate::internal::MethodCallBuilder;

#[doc(hidden)]
pub mod internal;
