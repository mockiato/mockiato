use syn::{BoundLifetimes, Lifetime, LifetimeDef};

/// Generates a for<...> clause from a list of given lifetimes
pub(crate) fn bound_lifetimes(lifetimes: Vec<Lifetime>) -> Option<BoundLifetimes> {
    if lifetimes.is_empty() {
        None
    } else {
        Some(BoundLifetimes {
            lifetimes: lifetimes.into_iter().map(LifetimeDef::new).collect(),
            ..Default::default()
        })
    }
}
