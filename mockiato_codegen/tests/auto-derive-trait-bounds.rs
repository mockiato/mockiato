#![feature(plugin)]
#![plugin(mockiato_codegen)]

use std::clone::Clone;
use std::fmt;
use std::marker::Copy;

#[mockable]
trait Foo: fmt::Debug + Clone + Copy {}

fn main() {
    let _assert_debug: &fmt::Debug = &FooMock;
    let _assert_clone = FooMock.clone();

    {
        let _mock = FooMock;
        let _copy = _mock;
        let _assert_copy = _mock;
    }
}
