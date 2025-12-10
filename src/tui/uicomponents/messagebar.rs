use std::{
    io::Error,
    time::{Duration, Instant},
};

use super::super::{Terminal, uicomponents::UIComponent};
use crate::prelude::*;

const DEFAULT_DURATION: Duration = Duration::new(5, 0);
const DEFAULT_MESSAGE: &str =
    "START: w | STOP: e | RELOAD: r | RESTART: t | ENABLE: y | DISABLE: u | EXIT: ctrl+q";

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::from(DEFAULT_MESSAGE),
            time: Instant::now(),
        }
    }
}

impl Message {
    fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Default)]
pub struct MessageBar {
    current_message: Message,
    needs_redraw: bool,
    cleared_after_expiry: bool,
}

impl UIComponent for MessageBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, _size: Size) {}

    fn draw(&mut self, origin_y: RowIdx) -> Result<(), Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }

        let message = if self.current_message.is_expired() {
            DEFAULT_MESSAGE
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin_y, message)
    }
}

impl MessageBar {
    pub fn redraw(&mut self) {
        self.set_needs_redraw(true);
    }
}
