use super::AnnotationType;
use crate::prelude::*;

// clippy::struct_field_names: struct field names cannot start with the name of the struct
#[derive(Clone, Copy, Debug)]
#[allow(clippy::struct_field_names)]
pub struct Annotation {
    pub annotation_type: AnnotationType,
    pub start: ByteIdx,
    pub end: ByteIdx,
}
