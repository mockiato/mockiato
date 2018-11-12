use mockiato_codegen::mockable;

#[mockable]
trait Foo {
    fn self_ref(&self, a: usize, b: Vec<String>);
    fn mut_self_ref(&mut self);
    fn captured_self(self: Box<Self>);
    fn self_ownership(self);
    fn arg_ref(&self, slice: &[u8]);
    fn arg_mut_ref(&self, buf: &mut [u8]);
    fn arg_ownership(&self, list: Vec<u32>);
}

fn main() {}
