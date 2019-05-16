use mockiato::mockable;
use std::collections::HashMap;

#[mockable]
trait Foo {
    fn bar(&self, hashmap: &HashMap<usize, usize>);
}

#[test]
fn works_with_hashmap() {
    let mut mock = FooMock::new();

    mock.expect_bar(|arg| arg.partial_eq_owned(HashMap::new()))
        .times(1);

    mock.bar(&HashMap::new());
}
