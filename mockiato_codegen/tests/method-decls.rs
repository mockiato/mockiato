use mockiato_codegen::mockable;

#[mockable]
trait Foo {
    fn self_ref(&self, a: usize, b: Vec<String>);
    fn mut_self_ref(&mut self);
    fn captured_self(self: Box<Self>);
    fn self_ownership(self);
    fn discarded_arg(&self, _: String);
}

fn main() {}
