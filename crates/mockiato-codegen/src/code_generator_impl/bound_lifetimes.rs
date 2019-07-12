use super::lifetime_rewriter::{IncrementalLifetimeGenerator, LifetimeRewriter};
use syn::visit_mut::visit_type_mut;
use syn::{BoundLifetimes, Lifetime, LifetimeDef, Type};

pub(super) fn rewrite_lifetimes_incrementally(mut ty: &mut Type) -> Option<BoundLifetimes> {
    let mut lifetime_rewriter = LifetimeRewriter::new(IncrementalLifetimeGenerator::default());
    visit_type_mut(&mut lifetime_rewriter, &mut ty);

    bound_lifetimes(lifetime_rewriter.generator.lifetimes)
}

/// Generates a for<...> clause from a list of given lifetimes
fn bound_lifetimes(lifetimes: Vec<Lifetime>) -> Option<BoundLifetimes> {
    if lifetimes.is_empty() {
        None
    } else {
        Some(BoundLifetimes {
            lifetimes: lifetimes.into_iter().map(LifetimeDef::new).collect(),
            ..BoundLifetimes::default()
        })
    }
}
