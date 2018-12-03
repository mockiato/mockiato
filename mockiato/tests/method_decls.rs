use mockiato::mockable;

use std::borrow::Cow;

#[mockable]
trait Foo {
    fn self_ref(&self, a: usize, b: Vec<String>);
    fn mut_self_ref(&mut self);
    fn captured_self(self: Box<Self>);
    fn self_ownership(self);
    fn arg_ref(&self, slice: &[u8]);
    fn arg_mut_ref(&self, buf: &mut [u8]);
    fn arg_ownership(&self, list: Vec<u32>);
    fn explicit_lifetime<'a>(&self, buf: &'a mut [u8]);
    fn explicit_lifetime_2<'a>(&self, names: Cow<'a, str>);
    fn where_clause<'a>(&self, buf: &'a mut [u8])
    where
        'a: 'static;
    fn multiple_ref_params(&self, name: (&str, &str));
}

fn main() {}
