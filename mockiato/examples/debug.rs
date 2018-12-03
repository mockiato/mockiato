use mockiato::mockable;
use std::fmt::Debug;

#[mockable]
trait Animal: Debug {
    fn make_sound(&self);
}

fn main() {
    let mut animal = AnimalMock::new();

    animal.expect_make_sound().times(1);
    animal.make_sound();

    // Prints something along the lines of:
    // AnimalMock {
    //     make_sound: Method {
    //         name: "AnimalMock::make_sound",
    //         calls: [
    //             ...
    //         ]
    //     }
    // }
    println!("{:#?}", animal);
}
