//! Shamelessly stolen from:
//! <https://github.com/SergioBenitez/Rocket/blob/master/core/codegen/tests/compiletest.rs>

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
use trybuild::TestCases;

#[test]
fn ui_tests() {
    let test_cases = TestCases::new();
    test_cases.compile_fail("tests/ui/*.rs");
}
