use mockiato_codegen::mockable;

#[mockable]
struct Foo {}

#[mockable]
enum Foo {}

#[mockable]
fn foo() {}

#[mockable]
type Foo = usize;
