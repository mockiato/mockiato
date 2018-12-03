#![deny(dead_code)]

use mockiato::mockable;

#[mockable]
trait Foo {
    fn say_hi(&self, name: &str);
}
