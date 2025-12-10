use std::{cmp::min, io::Error};

mod buffer;
mod highlighter;
mod searchinfo;

use super::super::{Mode, Terminal, TuiStatus, command::Move};
use super::UIComponent;
use crate::prelude::*;
use buffer::Buffer;
use highlighter::Highlighter;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: LineIdx,
    scroll_offset: RowIdx,
    search_query: Option<String>,
    hilight_selected_line: bool,
}

impl View {
    pub fn set_hilight_selected_line(&mut self, value: bool) {
        self.hilight_selected_line = value;
        self.set_needs_redraw(true);
    }

    pub fn load(&mut self) -> Result<(), Error> {
        let buffer = Buffer::load(self.size.width)?;

        self.buffer = buffer;
        self.set_needs_redraw(true);

        Ok(())
    }

    pub fn get_status(&self, mode: Mode) -> TuiStatus {
        TuiStatus {
            mode,
            total_lines: self.buffer.height(),
            current_line_idx: self.location,
        }
    }

    fn scroll(&mut self, to: RowIdx) {
        let Size { height, .. } = self.size;
        // let offset_changed =
        // self.scroll_offset = to;
        if to < self.scroll_offset {
            self.scroll_offset = to;
            // true
        } else if to >= self.scroll_offset.saturating_add(height) {
            self.scroll_offset = to.saturating_sub(height).saturating_add(1);
            // true
        }
        // else {
        //     false
        // };

        // if offset_changed {
        self.set_needs_redraw(true); // We are not using the cursor as position indicator, so we need to redraw the entire view.
        // }
    }

    fn location_to_position(&self) -> RowIdx {
        let row = self.location;

        debug_assert!(row.saturating_sub(1) <= self.buffer.height());

        row
    }

    fn scroll_location_into_view(&mut self) {
        debug_assert!(self.location.saturating_sub(1) <= self.buffer.height());

        self.scroll(self.location_to_position());
    }

    fn render_line(at: RowIdx, line_text: &str) -> Result<(), Error> {
        Terminal::print_row(at, line_text)
    }

    // pub fn cursor_position(&self) -> RowIdx {
    //     self.location_to_position()
    //         .saturating_sub(self.scroll_offset)
    // }

    fn snap_to_valid_line(&mut self) {
        self.location = min(self.location, self.buffer.height().saturating_sub(1));
    }

    fn move_up(&mut self, step: usize) {
        self.location = self.location.saturating_sub(step);
    }

    fn move_down(&mut self, step: usize) {
        self.location = self.location.saturating_add(step);
        self.snap_to_valid_line();
    }

    // review
    /// Edge case: No services
    pub fn scroll_to_start(&mut self) {
        self.move_up(self.location);
        self.scroll(self.location_to_position());
    }

    pub fn handle_move_command(&mut self, command: Move) {
        let Size { height, .. } = self.size;

        // This match moves the position, but does not check for all boundaries.
        // The final boundarline checking happens after the match statement.
        match command {
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1),
            Move::PageUp => self.move_up(height.saturating_sub(1)),
            Move::PageDown => self.move_down(height.saturating_sub(1)),
        }

        self.scroll_location_into_view();
    }

    pub fn filter(&mut self, query: &str) {
        self.buffer.filter(query);
        self.set_needs_redraw(true);
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
        let end_y = origin_row.saturating_add(self.size.height);

        let mut highlighter = Highlighter::new(self.search_query.as_deref(), self.location);

        for current_row in 0..end_y.saturating_add(self.scroll_offset) {
            self.buffer
                .highlight(current_row, &mut highlighter, self.hilight_selected_line);
        }

        for current_row in origin_row..end_y {
            // to get the correct line index, we have to take current_row (the absolute row on screen),
            // subtract origin_row to get the current row relative to the view (ranging from 0 to self.size.height)
            // and add the scroll offset
            let line_idx = current_row
                .saturating_sub(origin_row)
                .saturating_add(self.scroll_offset);

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
