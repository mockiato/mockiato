use mockiato_codegen::mockable;
use std::fmt;

#[mockable]
trait Foo: fmt::Debug {}

fn main() {
    let _assert_debug: &fmt::Debug = &FooMock::new();
}
