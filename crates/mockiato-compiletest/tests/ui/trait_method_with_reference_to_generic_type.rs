use mockiato::{mockable, partial_eq};
use std::fmt::{self, Debug, Display};

trait Foo {
    type Output: Display;
}

#[mockable]
trait TraitWithReferencesToGenericType<T, U, V>
where
    T: Display,
    U: Debug,
    V: Foo,
{
    fn reference_to_slice_of_t(&self, name: &[T]) -> String;

    fn container_type_with_reference_to_u(&self, name: Vec<&U>) -> String;

    fn reference_to_associated_type(&self, name: &V::Output) -> String;
}

fn main() {}
