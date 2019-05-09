use mockiato::mockable;

#[mockable(static_references = 1)]
trait TraitOne {}

#[mockable(static_references = "foo")]
trait TraitTwo {}

#[mockable(static_references, static_references)]
trait TraitThree {}

fn main() {}
