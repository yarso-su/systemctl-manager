use std::{cmp::min, io::Error};

mod buffer;
mod highlighter;
mod searchdirection;
mod searchinfo;

use super::super::{Mode, Terminal, TuiStatus, command::Move};
use super::UIComponent;
use crate::prelude::*;
use buffer::Buffer;
use highlighter::Highlighter;
use searchdirection::SearchDirection;
use searchinfo::SearchInfo;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size,
    location: LineIdx,
    scroll_offset: RowIdx,
    search_info: Option<SearchInfo>,
    hilight_selected_line: bool,
}

impl View {
    pub fn set_hilight_selected_line(&mut self, value: bool) {
        self.hilight_selected_line = value;
        self.set_needs_redraw(true);
    }

    pub fn get_selected_service_name(&self) -> Option<String> {
        self.buffer.get_selected_service_name(self.location)
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

    fn center_location(&mut self) {
        let Size { height, .. } = self.size;
        let vertical_mid = height.div_ceil(2);

        self.scroll_offset = self.location_to_position().saturating_sub(vertical_mid);
        self.set_needs_redraw(true);
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

    fn get_search_query(&self) -> Option<&String> {
        // let query = self
        //     .search_info
        //     .as_ref()
        //     .and_then(|search_info| search_info.query.as_ref());
        //
        // debug_assert!(
        //     query.is_some(),
        //     "Attempting to search with malformed searchinfo present"
        // );
        //
        // query

        self.search_info
            .as_ref()
            .and_then(|search_info| search_info.query.as_ref())
    }

    fn search_in_direction(&mut self, from: LineIdx, direction: SearchDirection) {
        // Review possible bug. SearchInfo is not cleared when exiting search mode
        if let Some(location) = self.get_search_query().and_then(|query| {
            if query.is_empty() {
                // debug_assert!(false, "none - al");
                None
            } else if direction == SearchDirection::Forward {
                self.buffer.search_forward(query, from)
            } else {
                self.buffer.search_backward(query, from)
            }
        }) {
            self.location = location;
            self.center_location();
        }

        self.set_needs_redraw(true);
    }

    pub fn search_next(&mut self) {
        self.search_in_direction(self.location.saturating_add(1), SearchDirection::Forward);
    }

    pub fn search_prev(&mut self) {
        self.search_in_direction(self.location.saturating_sub(1), SearchDirection::Backward);
    }

    pub fn search(&mut self, query: &str) {
        if let Some(search_info) = &mut self.search_info {
            search_info.query = Some(String::from(query));
        }

        self.search_in_direction(self.location, SearchDirection::default());
    }

    pub fn enter_search(&mut self) {
        self.search_info = Some(SearchInfo {
            prev_location: self.location,
            prev_scroll_offset: self.scroll_offset,
            query: None,
        });
    }

    pub fn exit_search(&mut self) {
        self.set_needs_redraw(true);
    }

    pub fn dismiss_search(&mut self) {
        if let Some(search_info) = &self.search_info {
            self.location = search_info.prev_location;
            self.scroll_offset = search_info.prev_scroll_offset;
            self.scroll_location_into_view(); // ensure the previous location is still visible even if the terminal has been resized
        }

        self.search_info = None;
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

        let query = self
            .search_info
            .as_ref()
            .and_then(|search_info| search_info.query.as_deref());
        let mut highlighter = Highlighter::new(query, self.location);

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
