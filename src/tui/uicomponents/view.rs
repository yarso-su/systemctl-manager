use std::io::Error;

mod buffer;
mod highlighter;
mod searchinfo;

use super::super::{Terminal, TuiStatus};
use super::UIComponent;
use crate::prelude::*;
use buffer::Buffer;
use highlighter::Highlighter;
use searchinfo::SearchInfo;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: LineIdx,
    scroll_offset: RowIdx,
    search_info: Option<SearchInfo>,
}

impl View {
    pub fn load(&mut self) -> Result<(), Error> {
        let buffer = Buffer::load()?;

        self.buffer = buffer;
        self.set_needs_redraw(true);

        Ok(())
    }

    pub fn get_status(&self) -> TuiStatus {
        TuiStatus {
            total_lines: self.buffer.height(),
            current_line_idx: self.location,
        }
    }

    fn scroll(&mut self, to: RowIdx) {
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset {
            self.scroll_offset = to;
            true
        } else if to >= self.scroll_offset.saturating_add(height) {
            self.scroll_offset = to.saturating_add(height).saturating_add(1);
            true
        } else {
            false
        };

        if offset_changed {
            self.set_needs_redraw(true);
        }
    }

    fn location_to_position(&self) -> RowIdx {
        let row = self.location;

        debug_assert!(row.saturating_sub(1) <= self.buffer.height());

        row
    }

    fn scroll_location_into_view(&mut self) {
        self.scroll(self.location_to_position());
    }

    fn render_line(at: RowIdx, line_text: &str) -> Result<(), Error> {
        Terminal::print_row(at, line_text)
    }

    pub fn cursor_position(&self) -> RowIdx {
        self.location_to_position()
            .saturating_sub(self.scroll_offset)
    }
}

impl UIComponent for View {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_location_into_view();
    }

    fn draw(&mut self, origin_row: RowIdx) -> Result<(), Error> {
        let Size { height, .. } = self.size;
        let end_y = origin_row.saturating_add(height);

        let query = self
            .search_info
            .as_ref()
            .and_then(|search_info| search_info.query.as_deref());
        let mut highlighter = Highlighter::new(query, self.location);

        for current_row in 0..end_y.saturating_add(self.scroll_offset) {
            self.buffer.highlight(current_row, &mut highlighter);
        }

        for current_row in origin_row..end_y {
            // to get the correct line index, we have to take current_row (the absolute row on screen),
            // subtract origin_row to get the current row relative to the view (ranging from 0 to self.size.height)
            // and add the scroll offset
            let line_idx = current_row
                .saturating_sub(origin_row)
                .saturating_add(self.scroll_offset);
            // let left = self.scroll_offset.col;
            // let right = self.scroll_offset.col.saturating_add(width);

            if let Some(annotated_string) =
                self.buffer.get_highlighted_string(line_idx, &highlighter)
            {
                Terminal::print_annotated_row(current_row, &annotated_string)?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        Ok(())
    }
}
