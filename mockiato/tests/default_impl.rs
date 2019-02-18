#![allow(clippy::blacklisted_name)]

use mockiato::mockable;

#[mockable]
trait IsFriendly {
    fn is_friendly(&self, _foo: &str) -> bool {
        true
    }
}
