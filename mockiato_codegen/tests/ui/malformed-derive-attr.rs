#![feature(plugin)]
#![plugin(mockiato_codegen)]

#[mockable(derive = "Foo")]
trait Foo {}
