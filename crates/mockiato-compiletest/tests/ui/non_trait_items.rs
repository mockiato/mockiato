use mockiato::mockable;

#[mockable]
struct Struct {}

#[mockable]
enum Enum {}

#[mockable]
fn function() {}

#[mockable]
type TypeAlias = usize;

fn main() {}
