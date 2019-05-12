use mockiato::mockable;
use std::fmt::Debug;

#[mockable]
trait Greeter: Debug {
    fn greet_person(&self, person: &Person) -> String;
}

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    let person = Person {
        name: String::from("Name"),
        age: 30,
    };
    let mut greeter = GreeterMock::new();

    greeter
        .expect_greet_person(|f| f.partial_eq(&person))
        .times(1)
        .returns(String::from("Hello Name"));
    greeter.greet_person(&person);

    // Prints something along the lines of:
    // GreeterMock {
    //     greet_person: Method {
    //         name: "GreeterMock::greet_person",
    //         calls: [
    //             ...
    //         ]
    //     }
    // }
    println!("{:#?}", greeter);
}
