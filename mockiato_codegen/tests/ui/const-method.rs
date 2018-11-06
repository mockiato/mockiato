use mockiato_codegen::mockable;

#[mockable]
trait Foo {
    const fn bar(&self, baz: u64);
}

fn main() {}
