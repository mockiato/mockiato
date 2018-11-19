use super::constant::arguments_lifetime;
use syn::visit_mut::{visit_type_reference_mut, VisitMut};
use syn::{Lifetime, TypeReference};

pub(super) trait LifetimeGenerator {
    fn generate_lifetime(&mut self) -> Lifetime;
}

/// Replaces all lifetimes in the given AST with a lifetime provided by a [`LifetimeGenerator`].
/// It also gives explicit lifetimes to references without lifetimes
pub(super) struct LifetimeRewriter<T>
where
    T: LifetimeGenerator,
{
    pub(super) generator: T,
}

impl<T> LifetimeRewriter<T>
where
    T: LifetimeGenerator,
{
    pub(super) fn new(generator: T) -> Self {
        Self { generator }
    }
}

impl<T> VisitMut for LifetimeRewriter<T>
where
    T: LifetimeGenerator,
{
    fn visit_lifetime_mut(&mut self, lifetime: &mut Lifetime) {
        *lifetime = self.generator.generate_lifetime();
    }

    fn visit_type_reference_mut(&mut self, type_reference: &mut TypeReference) {
        visit_type_reference_mut(self, type_reference);

        if type_reference.lifetime.is_none() {
            type_reference.lifetime = Some(self.generator.generate_lifetime());
        }
    }
}

/// Replaces all lifetimes with the same lifetime
#[derive(Default)]
pub struct UniformLifetimeGenerator {
    // Indicates that the rewriter found at least one lifetime
    pub has_lifetimes: bool,
}

impl LifetimeGenerator for UniformLifetimeGenerator {
    fn generate_lifetime(&mut self) -> Lifetime {
        self.has_lifetimes = true;
        arguments_lifetime()
    }
}
