use mockiato::mockable;
use std::fmt::Debug;

#[mockable]
trait Animal: Debug {
    fn make_sound(&self);
}

fn main() {
    let mut animal = AnimalMock::default();

    animal.expect_make_sound().times(1);
    animal.make_sound();
}
