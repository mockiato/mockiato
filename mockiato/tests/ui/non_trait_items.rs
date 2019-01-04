use mockiato::mockable;

#[mockable]
struct Foo {}

#[mockable]
enum Foo {}

#[mockable]
fn foo() {}

#[mockable]
type Foo = usize;

fn main() {}
