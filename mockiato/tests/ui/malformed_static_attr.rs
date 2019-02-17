use mockiato::mockable;

#[mockable(static = 1)]
trait Foo {}

#[mockable(static = "foo")]
trait Foo {}

#[mockable(static, static)]
trait Foo {}

fn main() {}
