use mockiato::mockable;

#[mockable]
trait Foo {
    fn bar(&self, _: u64);
}

fn main() {}
