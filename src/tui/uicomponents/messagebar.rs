use std::{
    io::Error,
    time::{Duration, Instant},
};

use super::super::{Terminal, uicomponents::UIComponent};
use crate::prelude::*;

const DEFAULT_DURATION: Duration = Duration::new(3, 0);
const DEFAULT_MESSAGE_LEFT: &str = "alternate loaded/all: f";
const DEFAULT_MESSAGE_RIGHT: &str = "show keys: o | help: p | exit: ctrl+q";

struct Message {
    text: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            text: String::new(),
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
    size: Size,
}

impl UIComponent for MessageBar {
    fn set_needs_redraw(&mut self, value: bool) {
        self.needs_redraw = value;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && self.current_message.is_expired()) || self.needs_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn draw(&mut self, origin_y: RowIdx) -> Result<(), Error> {
        if self.current_message.is_expired() {
            self.cleared_after_expiry = true;
        }

        let message = if self.current_message.is_expired() || self.current_message.text.is_empty() {
            let remainder_len = self.size.width.saturating_sub(DEFAULT_MESSAGE_LEFT.len());
            &format!("{DEFAULT_MESSAGE_LEFT}{DEFAULT_MESSAGE_RIGHT:>remainder_len$}")
        } else {
            &self.current_message.text
        };

        Terminal::print_row(origin_y, message)
    }
}

impl MessageBar {
    pub fn update_message(&mut self, new_message: &str) {
        self.current_message = Message {
            text: new_message.to_string(),
            time: Instant::now(),
        };

        self.cleared_after_expiry = false;
        self.set_needs_redraw(true);
    }

    pub fn clear_message(&mut self) {
        self.current_message = Message::default();
        self.cleared_after_expiry = true;
        self.set_needs_redraw(true);
    }

    pub fn redraw(&mut self) {
        self.set_needs_redraw(true);
    }
}
