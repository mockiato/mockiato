pub(crate) use self::code_generator_impl::CodeGeneratorImpl;
use crate::parse::method_decl::MethodDecl;
use crate::parse::trait_decl::TraitDecl;
use proc_macro2::TokenStream;
use std::fmt::Debug;
use syn::{Generics, Ident, Path, Type};

pub(crate) mod arguments;
pub(crate) mod arguments_matcher;
mod bound_lifetimes;
mod code_generator_impl;
mod constant;
mod debug_impl;
mod drop_impl;
mod generics;
mod lifetime_rewriter;
mod mock_struct;
mod trait_impl;
mod util;
mod visibility;

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct GenerateMockOptions {
    pub(crate) custom_struct_ident: Option<Ident>,
    pub(crate) force_static_lifetimes: bool,
    pub(crate) custom_trait_path: Option<Path>,
}

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct GenerateMockParameters {
    pub(crate) mock_struct_ident: Ident,
    pub(crate) mod_ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) trait_path: Path,
    pub(crate) methods: Vec<MethodDeclMetadata>,
}

#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct MethodDeclMetadata {
    pub(crate) method_decl: MethodDecl,
    pub(crate) arguments_struct_ident: Ident,
    pub(crate) arguments_matcher_struct_ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) return_type: Type,
}

pub(crate) trait CodeGenerator: Debug {
    fn generate(&self, trait_decl: &TraitDecl, options: GenerateMockOptions) -> TokenStream;
}
