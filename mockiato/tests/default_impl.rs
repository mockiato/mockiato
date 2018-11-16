use mockiato::mockable;

#[mockable]
trait IsFriendly {
    fn is_friendly(&self, _foo: &str) -> bool {
        true
    }
}
