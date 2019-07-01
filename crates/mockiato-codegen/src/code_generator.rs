use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::{Ident, Path};

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct GenerateOptions {
    pub(crate) custom_struct_ident: Option<Ident>,
    pub(crate) force_static_lifetimes: bool,
    pub(crate) custom_trait_path: Option<Path>,
}

pub(crate) trait CodeGenerator: Debug {
    fn generate(&self, trait_decl: &TraitDecl, options: GenerateOptions) -> TokenStream;
}
