#![feature(const_fn)]

use mockiato::mockable;

#[mockable]
trait Foo {
    const fn bar(&self, baz: u64);
}

fn main() {}
