use mockiato_codegen::mockable;

#[mockable]
trait Foo {
    fn bar(baz: u64);
}

fn main() {}
