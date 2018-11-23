use mockiato_codegen::mockable;
use std::fmt::{Debug, Display};

#[mockable]
trait Greeter: Debug {
    fn greet(&self, name: &Display) -> String;
}

fn main() {
    let greeter = GreeterMock::new();

    println!("{:?}", greeter);
}
