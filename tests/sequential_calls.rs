use mockiato::mockable;
use std::collections::HashMap;

#[mockable]
trait Foo {
    fn bar(&self) -> bool;
}

#[test]
fn sequential_calls() {
    let mut mock = FooMock::new();

    mock.expect_bar().returns(true);
    mock.expect_bar().returns(false);
    mock.expect_bar().returns(true);
    mock.expect_bar_calls_in_order();

    assert!(mock.bar());
    assert!(!mock.bar());
    assert!(mock.bar());
}
