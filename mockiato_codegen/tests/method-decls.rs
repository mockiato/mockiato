use mockiato_codegen::mockable;

#[mockable]
trait Foo {
    fn bar(&self, a: usize, b: Vec<String>);
    fn baz(self: Box<Self>);
    fn quz(self);
}

fn main() {}
