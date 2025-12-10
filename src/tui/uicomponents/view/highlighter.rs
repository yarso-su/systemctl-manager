use std::collections::HashMap;

use super::{
    super::super::{Annotation, AnnotationType},
    buffer::Service,
};
use crate::prelude::*;

#[derive(Default)]
pub struct Highlighter<'a> {
    matched_word: Option<&'a str>,
    location: LineIdx,
    highlights: HashMap<LineIdx, Vec<Annotation>>,
}

impl<'a> Highlighter<'a> {
    pub fn new(matched_word: Option<&'a str>, location: LineIdx) -> Self {
        Self {
            matched_word,
            location,
            highlights: HashMap::new(),
        }
    }

    pub fn highlight(&mut self, idx: LineIdx, service: &Service, highligh_selected_line: bool) {
        let mut result = Vec::new();

        let annotation_type = if self.location == idx && highligh_selected_line {
            AnnotationType::SelectedMatch
        } else {
            AnnotationType::Match
        };

        if let Some(matched_word) = self.matched_word
            && !matched_word.is_empty()
        {
            service
                .find_all(matched_word, 0..service.len())
                .iter()
                .for_each(|start| {
                    result.push(Annotation {
                        annotation_type,
                        start: *start,
                        end: start.saturating_add(matched_word.len()),
                    });
                });
        }

        let result = if self.location == idx && highligh_selected_line {
            let mut char_idx = 0;
            let mut annotation_idx = 0;
            let mut selected_annotations = result.clone();

            while char_idx < service.len() {
                if let Some(annotation) = result.get(annotation_idx) {
                    if char_idx < annotation.start {
                        selected_annotations.insert(
                            annotation_idx,
                            Annotation {
                                annotation_type: AnnotationType::Selected,
                                start: char_idx,
                                end: annotation.start.saturating_sub(1),
                            },
                        );
                    }

                    char_idx = annotation.end.saturating_add(1);
                    annotation_idx = annotation_idx.saturating_add(1);
                } else {
                    selected_annotations.push(Annotation {
                        annotation_type: AnnotationType::Selected,
                        start: char_idx,
                        end: service.len(),
                    });
                    break;
                }
            }

            selected_annotations
        } else {
            result
        };

        self.highlights.insert(idx, result);
    }

    pub fn get_annotations(&self, idx: LineIdx) -> Option<&Vec<Annotation>> {
        self.highlights.get(&idx)
    }
}
