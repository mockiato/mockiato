use mockiato::mockable;

#[mockable]
trait Greeter {
    fn greet<T>(&self, baz: T);
}

fn main() {}
