use std::{cmp::min, io::Error};

use super::super::{
    Terminal,
    command::Edit::{self, DeleteBackward, Insert},
    uicomponents::UIComponent,
};
use crate::prelude::*;

const PROMT: &str = "> ";

#[derive(Default)]
pub struct FilterBar {
    value: String,
    needs_redraw: bool,
    size: Size,
}

impl UIComponent for FilterBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin: RowIdx) -> Result<(), Error> {
        let message = format!("{}{}", PROMT, self.value);
        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };

        Terminal::print_row(origin, &to_print)
    }
}

impl FilterBar {
    pub fn handle_edit_command(&mut self, command: Edit) {
        match command {
            Insert(ch) => self.value.push(ch),
            Edit::Delete | Edit::InsertNewLine => {}
            DeleteBackward => {
                self.value.pop();
                // debug_assert!(false, "{}", self.value);
            }
        }

        self.set_needs_redraw(true);
    }

    pub fn caret_position_col(&self) -> ColIdx {
        let max_width = PROMT.len().saturating_add(self.value.len());

        min(max_width, self.size.width)
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }

    pub fn clear_value(&mut self) {
        self.value.clear();
        self.set_needs_redraw(true);
    }
}
