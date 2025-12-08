use std::{io::Error, process::Command};

mod service;

use super::{super::super::AnnotatedString, SearchResultHighlighter};
use crate::prelude::*;
pub use service::Service;

#[derive(Default)]
pub struct Buffer {
    services: Vec<Service>,
}

impl Buffer {
    pub fn height(&self) -> usize {
        self.services.len()
    }

    pub fn load() -> Result<Self, Error> {
        let output = Command::new("systemctl")
            .args([
                "list-units",
                "--type=service",
                "--all",
                "--no-pager",
                "--no-legend",
                "--plain",
            ])
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut services = Vec::new();

        for line in stdout.lines() {
            if let Some(service) = Service::new(line) {
                services.push(service);
            }
        }

        Ok(Self { services })
    }

    pub fn get_highlighted_string(
        &self,
        line_idx: LineIdx,
        highlighter: &SearchResultHighlighter,
    ) -> Option<AnnotatedString> {
        self.services
            .get(line_idx)
            .map(|service| service.get_annotated_string(highlighter.get_annotations(line_idx)))
    }

    pub fn highlight(&mut self, idx: LineIdx, highlighter: &mut SearchResultHighlighter) {
        if let Some(line) = self.services.get(idx) {
            highlighter.highlight(idx, line);
        }
    }
}
