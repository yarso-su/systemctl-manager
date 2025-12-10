use std::io::Error;

use super::super::{Terminal, TuiStatus, UIComponent};
use crate::prelude::*;

#[derive(Default)]
pub struct StatusBar {
    current_status: TuiStatus,
    needs_redraw: bool,
    size: Size,
}

impl UIComponent for StatusBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin_row: RowIdx) -> Result<(), Error> {
        let mode_text = self.current_status.mode.to_string();
        let position_indicator = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(mode_text.len());
        let status = format!("{mode_text}{position_indicator:>remainder_len$}");

        // Only print out the status if it fits. Otherwise write out an empty string to ensure the row is cleared
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };

        Terminal::print_inverted_row(origin_row, &to_print)?;

        Ok(())
    }
}

impl StatusBar {
    pub fn update_status(&mut self, new_status: TuiStatus) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.set_needs_redraw(true);
        }
    }
}
