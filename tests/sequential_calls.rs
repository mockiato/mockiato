use mockiato::mockable;

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

#[test]
fn sequential_calls_work_with_expected_call_amount_with_no_upper_limit() {
    let mut mock = FooMock::new();

    mock.expect_bar_calls_in_order();
    mock.expect_bar().returns(true).times(3..);

    for _ in 0..3 {
        assert!(mock.bar());
    }
}
