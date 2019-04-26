use syn::{Ident, Path};

pub(crate) trait PathExt {
    fn first_segment_as_ident(&self) -> Option<&Ident>;
}

impl PathExt for Path {
    fn first_segment_as_ident(&self) -> Option<&Ident> {
        Some(&self.segments.iter().nth(0)?.ident)
    }
}
