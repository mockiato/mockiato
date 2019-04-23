use syn::{Ident, Path};

pub(crate) trait PathExt {
    fn first_segment_as_ident(&self) -> Option<&Ident>;
}

impl PathExt for Path {
    fn first_segment_as_ident(&self) -> Option<&Ident> {
        if self.segments.is_empty() {
            None
        } else {
            Some(&self.segments[0].ident)
        }
    }
}
