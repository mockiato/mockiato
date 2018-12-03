use mockiato::mockable;

#[mockable]
pub trait Animal {
    fn make_sound(&self);
}
