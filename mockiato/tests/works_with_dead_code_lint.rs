#![deny(dead_code)]

use mockiato_codegen::mockable;

#[mockable]
trait Foo {
    fn say_hi(&self, name: &str);
}
