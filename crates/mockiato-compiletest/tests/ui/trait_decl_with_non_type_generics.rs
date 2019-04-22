use mockiato::mockable;

#[mockable]
trait TraitWithLifetime<'a> {}

#[mockable]
trait TraitWithLifetimeAndGenericParam<'a, B> {}

fn main() {}
