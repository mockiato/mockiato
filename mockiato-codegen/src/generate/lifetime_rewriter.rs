use super::constant::argument_lifetime;
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
pub(crate) struct UniformLifetimeGenerator {
    // Indicates that the rewriter found at least one lifetime
    has_lifetimes: bool,
    lifetime: Lifetime,
}

impl UniformLifetimeGenerator {
    pub(crate) fn new(lifetime: Lifetime) -> Self {
        Self {
            lifetime,
            has_lifetimes: false,
        }
    }

    pub(crate) fn has_lifetimes(&self) -> bool {
        self.has_lifetimes
    }
}

impl LifetimeGenerator for UniformLifetimeGenerator {
    fn generate_lifetime(&mut self) -> Lifetime {
        self.has_lifetimes = true;
        self.lifetime.clone()
    }
}

/// Replaces all lifetimes in the given AST with auto-generated lifetimes that
/// can be used in a for<...> clause.
/// It also gives explicit lifetimes to references without lifetimes
#[derive(Default)]
pub(crate) struct IncrementalLifetimeGenerator {
    pub(crate) lifetimes: Vec<Lifetime>,
}

impl LifetimeGenerator for IncrementalLifetimeGenerator {
    fn generate_lifetime(&mut self) -> Lifetime {
        // The only requirement for this lifetime is that it's unique.
        // The fixed prefix is arbitrary.
        let lifetime = argument_lifetime(self.lifetimes.len());
        self.lifetimes.push(lifetime.clone());
        lifetime
    }
}
