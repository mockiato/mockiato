#![feature(custom_attribute, plugin)]
#![plugin(mockiato_codegen)]

use std::fmt::Debug;

#[mockable]
trait Foo: Debug {}

fn main() {}
