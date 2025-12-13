use std::{cmp::min, ops::Range};

use super::{super::super::super::Annotation, AnnotatedString};
use crate::prelude::*;

#[derive(Clone)]
pub struct Service {
    string: String,
}

impl Service {
    pub fn new(line: String) -> Self {
        Self { string: line }
    }

    pub fn len(&self) -> usize {
        self.string.len()
    }

    pub fn extract_name(&self) -> String {
        self.string
            .split_once(".service")
            .map_or_else(String::new, |(name, _)| name.to_string())
    }

    pub fn starts_with(&self, query: &str) -> bool {
        self.string.starts_with(query)
    }

    pub fn contains(&self, query: &str) -> bool {
        self.string.contains(query)
    }

    pub fn find_all(&self, query: &str, range: Range<ByteIdx>) -> Vec<ByteIdx> {
        let end = min(range.end, self.string.len());
        let start = range.start;

        debug_assert!(start <= end);
        debug_assert!(start <= self.string.len());

        self.string.get(start..end).map_or_else(Vec::new, |substr| {
            substr
                .match_indices(query)
                .map(|(relative_start_idx, _)| relative_start_idx.saturating_add(start))
                .collect()
        })
    }

    pub fn get_annotated_string(&self, annotations: Option<&Vec<Annotation>>) -> AnnotatedString {
        let mut result = AnnotatedString::from(&self.string);

        if let Some(annotations) = annotations {
            for annotation in annotations {
                result.add_annotation(annotation.annotation_type, annotation.start, annotation.end);
            }
        }

        result
    }
}
