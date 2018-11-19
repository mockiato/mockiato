use mockiato_codegen::mockable;
use std::fmt::{Debug, Display};

#[mockable]
trait Greeter: Debug {
    fn greet(&self, name: &Display) -> String;
}

fn main() {
    // TODO: this example needs to be updated
    // as soon as methods mocking is implemented.
    let greeter = GreeterMock::new();

    println!("{:?}", greeter);
}
