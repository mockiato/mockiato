#![feature(plugin)]
#![plugin(mockiato_codegen)]

#[mockable("Bar")]
trait Foo {}
