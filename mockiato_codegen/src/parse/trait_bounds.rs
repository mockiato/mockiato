use super::trait_decl::TraitDecl;
use crate::syntax::ast::{GenericBound, Path, PathSegment};
use crate::syntax_pos::Span;

#[derive(Debug)]
pub(crate) struct TraitBounds(pub(crate) Vec<TraitBound>);

#[derive(Debug)]
pub(crate) struct TraitBound {
    pub(crate) path: Path,
    pub(crate) span: Span,
}

fn path_without_generic_args(path: &Path) -> Path {
    let segments = path
        .segments
        .iter()
        .map(|segment| PathSegment::from_ident(segment.ident))
        .collect();

    Path {
        span: path.span,
        segments,
    }
}

impl TraitBounds {
    pub(crate) fn parse(trait_decl: &TraitDecl) -> Self {
        TraitBounds(
            trait_decl
                .generic_bounds
                .iter()
                .filter_map(|generic_bound| {
                    if let GenericBound::Trait(poly_trait_ref, _trait_bound_modifier) =
                        generic_bound
                    {
                        Some(TraitBound {
                            path: path_without_generic_args(&poly_trait_ref.trait_ref.path),
                            span: poly_trait_ref.trait_ref.path.span,
                        })
                    } else {
                        None
                    }
                }).collect(),
        )
    }
}
