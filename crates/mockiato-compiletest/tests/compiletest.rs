#[cfg(rustc_is_nightly)]
#[test]
fn ui_tests() {
    let test_cases = trybuild::TestCases::new();
    test_cases.compile_fail("tests/ui/*.rs");
}
