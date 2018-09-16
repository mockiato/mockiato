#![feature(custom_attribute, plugin)]
#![plugin(mockiato_codegen)]

use std::fmt::Display;

trait Debug {}

// #[mockable(derive(Debug))]
trait Greeter<D>
where
    D: Display,
{
    fn say_hello(&self, name: D) -> String;
}

// #[mockable(derive(Debug))]
trait PoliteGreeter<D>: Greeter<D>
where
    D: Display,
{
    fn say_hello_politely(&self, name: D) -> String;
}

// #[derive(Debug)]
// #[mockable]
// struct GreeterMock {}

//impl<D> Greeter<D> for GreeterMock
//where
//    D: Display,
//{
//    fn say_hello(&self, _name: D) -> String {
//        unimplemented!();
//    }
//}
#[test]
fn test() {
    //    let _mock = GreeterMock;
}
