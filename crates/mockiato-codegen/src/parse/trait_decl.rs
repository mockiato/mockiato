use crate::parse::method_decl::MethodDecl;
use crate::result::Result;
use proc_macro2::Span;

use std::fmt::Debug;
use syn::punctuated::Punctuated;

use syn::{Generics, Ident, ItemTrait, Token, TypeParamBound, Visibility};

#[derive(Clone)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub(crate) struct TraitDecl {
    pub(crate) visibility: Visibility,
    pub(crate) span: Span,
    pub(crate) ident: Ident,
    pub(crate) generics: Generics,
    pub(crate) unsafety: Option<Token![unsafe]>,
    pub(crate) supertraits: Punctuated<TypeParamBound, Token![+]>,
    pub(crate) methods: Vec<MethodDecl>,
}

pub(crate) trait TraitDeclParser: Debug {
    fn parse(&self, item: ItemTrait) -> Result<TraitDecl>;
}
