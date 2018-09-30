#![feature(plugin)]
#![plugin(mockiato_codegen)]

trait Bar {}

#[mockable]
trait Foo: Bar + std::fmt::Debug {}

#[mockable]
trait Baz: std::fmt::Display {}
