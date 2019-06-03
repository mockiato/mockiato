//! A strict, yet friendly mocking library for Rust 2018
//!
//! # Examples
//!
//! ```
//! use mockiato::mockable;
//!
//! # const IGNORED: &str = "
//! #[cfg_attr(test, mockable)]
//! # ";
//! # #[mockable]
//! trait Greeter {
//!     fn greet(&self, name: &str) -> String;
//! }
//!
//! let mut greeter = GreeterMock::new();
//!
//! greeter
//!     .expect_greet(|arg| arg.partial_eq("Jane"))
//!     .times(1..)
//!     .returns(String::from("Hello Jane"));
//!
//! assert_eq!("Hello Jane", greeter.greet("Jane"));
//! ```
//!
//! # Configuring Expected Calls
//!
//! Each method on the trait receives two companion methods on the mock struct:
//!
//! ## `expect_<method_name>`
//!
//! Registers an expected call to the mocked method. Exactly one call is expected by default.
//! The order in which these methods are called is irrelevant, unless configured otherwise.
//!
//! It has the same amount of arguments as the mocked method.
//! Each argument accepts a closure that is invoked with a reference to [`Argument`], which lets
//! you create different argument matchers.
//!
//! This method returns a [`MethodCallBuilder`] which allows for further customization of an expected call's behavior.
//!
//! ```
//! use mockiato::mockable;
//!
//! # const IGNORED: &str = "
//! #[cfg_attr(test, mockable)]
//! # ";
//! # #[mockable]
//! trait MessageSender {
//!     fn send_message(&self, recipient: &str, message: &str);
//! }
//!
//! let mut message_sender = MessageSenderMock::new();
//! message_sender
//!     .expect_send_message(|arg| arg.partial_eq("Paul"), |arg| arg.any())
//!     .times(..)
//!     .returns(());
//! ```
//!
//! ## `expect_<method_name>_calls_in_order`
//!
//! Configures the mocked method so that the expected calls are processed sequentially.
//! When this is enabled, the calls to the mocked method must be in the same order as the `expect_` methods were called.
//!
//! ```
//! # use mockiato::mockable;
//! #
//! # const IGNORED: &str = "
//! #[cfg_attr(test, mockable)]
//! # ";
//! # #[mockable]
//! # trait MessageSender {
//! #     fn send_message(&self, recipient: &str, message: &str);
//! # }
//! #
//! # let mut message_sender = MessageSenderMock::new();
//! message_sender.expect_send_message_calls_in_order();
//! ```
//!
//! # Call Verification
//! Mockiato automatically verifies that all expected calls were made when the mock goes out of scope.
//! The mock panics when a method is called that was not configured, or if the parameters did not match.
//! ```no_run
//! use mockiato::mockable;
//!
//! # const IGNORED: &str = "
//! #[cfg_attr(test, mockable)]
//! # ";
//! # #[mockable]
//! trait Greeter {
//!     fn greet(&self, name: &str) -> String;
//! }
//!
//! {
//!     let mut greeter = GreeterMock::new();
//!
//!     greeter
//!         .expect_greet(|arg| arg.partial_eq("Doe"))
//!         .times(1..)
//!         .returns(String::from("Hello Doe"));
//!
//!     assert_eq!("Hello Jane", greeter.greet("Jane"));
//!     //                               ^^^^^^^^^^^^^
//!     //                               This call was not configured, which results in a panic
//!
//!     //      The mock verifies that all expected calls have been made
//!     // <--  and panics otherwise
//! }
//! ```

#![cfg_attr(rustc_is_nightly, feature(doc_cfg, external_doc, specialization))]
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

#[cfg(any(not(rustc_is_nightly), not(rustdoc)))]
pub use mockiato_codegen::mockable;

#[cfg(all(rustc_is_nightly, rustdoc))]
#[macro_export]
/// Generates a mock struct from a trait.
///
/// # Parameters
///
/// ## `static_references`
/// Forces values stored in argument matchers to be `'static`. This is used when the mock needs to satisfy
/// `'static` e.g. when downcasting the mocked trait to a concrete implementation using the `Any` trait.
/// There is an [example] available on how to do this.
///
/// [example]: https://github.com/myelin-ai/mockiato/blob/master/examples/downcasting.rs
///
/// ```
/// use mockiato::mockable;
/// use std::any::Any;
///
/// #[cfg_attr(test, mockable(static_references))]
/// pub trait Animal: Any {
///     fn make_sound(&self);
/// }
/// ```
///
/// ## `name`
/// Sets a custom name for the mock struct instead of the default.
/// ```
/// use mockiato::mockable;
///
/// #[cfg_attr(test, mockable(name = "CuteAnimalMock"))]
/// trait Animal {
///     fn make_sound(&self);
/// }
/// ```
macro_rules! mockable {
    () => {};
}

#[cfg_attr(rustc_is_nightly, doc(include = "../readme.md"))]
mod test_readme {}

pub use crate::argument::Argument;
pub use crate::expected_calls::ExpectedCalls;
pub use crate::method_call::MethodCallBuilder;

mod argument;
mod arguments;
mod default_return_value;
mod expected_calls;
mod fmt;
#[doc(hidden)]
pub mod internal;
mod matcher;
mod method;
mod method_call;
mod return_value;
