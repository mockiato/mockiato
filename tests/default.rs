mod foo {
    use mockiato::mockable;

    #[mockable]
    pub(super) trait Bar {}
}

#[test]
fn default_works() {
    let _ = foo::BarMock::default();
}
