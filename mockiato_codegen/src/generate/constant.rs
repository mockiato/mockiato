use proc_macro2::Span;
use syn::Lifetime;

pub(super) fn arguments_lifetime() -> Lifetime {
    const LIFETIME_NAME: &str = "'__mockiato_args";

    Lifetime::new(LIFETIME_NAME, Span::call_site())
}
