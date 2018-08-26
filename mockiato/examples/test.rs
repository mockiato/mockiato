#![feature(custom_attribute, plugin)]
#![plugin(mockiato_codegen)]

use std::fmt::{Debug, Display};

#[mockable]
trait Greeter<D>: Debug
where
    D: Display,
{
    fn say_hello(&self, name: D) -> String;
}

#[derive(Debug)]
// #[mockable]
struct GreeterMock {}

impl<D> Greeter<D> for GreeterMock
where
    D: Display,
{
    fn say_hello(&self, _name: D) -> String {
        unimplemented!();
    }
}

fn main() {}
