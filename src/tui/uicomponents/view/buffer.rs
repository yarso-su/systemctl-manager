use std::{io::Error, process::Command};

mod service;

use super::{super::super::AnnotatedString, Highlighter};
use crate::prelude::*;
pub use service::Service;

#[derive(Default)]
pub struct Buffer {
    services: Vec<Service>,
    filtered: Option<Vec<Service>>,
}

impl Buffer {
    fn get_default_collection(&self) -> &Vec<Service> {
        &self.services
    }

    fn get_active_collection(&self) -> &Vec<Service> {
        if let Some(filtered) = &self.filtered {
            filtered
        } else {
            &self.services
        }
    }

    pub fn height(&self) -> usize {
        self.get_active_collection().len()
    }

    pub fn load(terminal_width: usize) -> Result<Self, Error> {
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
            let fill = " ".repeat(terminal_width.saturating_sub(line.len()));
            let mut line = String::from(line);
            line.push_str(&fill);

            if let Some(service) = Service::new(line) {
                services.push(service);
            }
        }

        Ok(Self {
            services,
            filtered: None,
        })
    }

    pub fn get_highlighted_string(
        &self,
        line_idx: LineIdx,
        highlighter: &Highlighter,
    ) -> Option<AnnotatedString> {
        self.get_active_collection()
            .get(line_idx)
            .map(|service| service.get_annotated_string(highlighter.get_annotations(line_idx)))
    }

    // Review performance, it clones the whole buffer each time the user types a character
    pub fn filter(&mut self, query: &str) {
        if query.is_empty() {
            self.filtered = None;
            return;
        }

        let mut services = Vec::new();
        for idx in 0..self.get_default_collection().len() {
            if let Some(line) = self.services.get(idx)
                && line.starts_with(query)
            {
                services.push(line.clone());
            }
        }

        self.filtered = Some(services);
    }

    pub fn highlight(
        &mut self,
        idx: LineIdx,
        highlighter: &mut Highlighter,
        highligh_selected_line: bool,
    ) {
        if let Some(line) = self.get_active_collection().get(idx) {
            highlighter.highlight(idx, line, highligh_selected_line);
        }
    }

    pub fn search_forward(&self, query: &str, from: LineIdx) -> Option<LineIdx> {
        if query.is_empty() {
            return None;
        }

        for (line_idx, service) in self
            .get_active_collection()
            .iter()
            .enumerate()
            .cycle()
            .skip(from)
            .take(self.get_active_collection().len())
        {
            if service.contains(query) {
                return Some(line_idx);
            }
        }

        None
    }

    pub fn search_backward(&self, query: &str, from: LineIdx) -> Option<LineIdx> {
        if query.is_empty() {
            return None;
        }

        for (line_idx, service) in self
            .get_active_collection()
            .iter()
            .enumerate()
            .rev()
            .cycle()
            .skip(
                self.get_active_collection()
                    .len()
                    .saturating_sub(from)
                    .saturating_sub(1),
            )
            .take(self.get_active_collection().len())
        {
            if service.contains(query) {
                return Some(line_idx);
            }
        }

        None
    }
}
