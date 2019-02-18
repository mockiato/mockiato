use mockiato::mockable;

#[mockable(static_references = 1)]
trait Foo {}

#[mockable(static_references = "foo")]
trait Foo {}

#[mockable(static_references, static_references)]
trait Foo {}

fn main() {}
