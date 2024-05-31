#![deny(dead_code)]

use mockiato::mockable;

#[mockable]
trait Foo {
    fn say_hi(&self, name: &str);
}

struct FooImpl;

impl Foo for FooImpl {
    fn say_hi(&self, _name: &str) {}
}

fn main() {
    let foo = FooImpl;
    foo.say_hi("test");
}
