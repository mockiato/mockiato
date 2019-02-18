use mockiato::mockable;

#[mockable(name = 1)]
trait Foo {}

#[mockable(name(FooMock))]
trait Bar {}

#[mockable(name)]
trait Baz {}

fn main() {}
