extern crate mockiato_codegen;
use mockiato_codegen::mockable;

#[mockable("Bar")]
trait Foo {}

fn main() {}
