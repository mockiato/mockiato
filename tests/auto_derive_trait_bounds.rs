use mockiato::mockable;
use std::fmt;

#[mockable]
trait Foo: fmt::Debug {}

fn main() {
    let _assert_debug: &dyn fmt::Debug = &FooMock::new();
}
