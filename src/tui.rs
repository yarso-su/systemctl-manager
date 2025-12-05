use crossterm::event::{Event, KeyCode::Char, read};
use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

mod annotatedstring;
mod annotation;
mod annotationtype;
mod terminal;

use crate::prelude::*;
use annotatedstring::AnnotatedString;
use annotation::Annotation;
use annotationtype::AnnotationType;
use terminal::Terminal;

#[derive(Eq, PartialEq, Default, Debug)]
enum Mode {
    #[default]
    Search,
    Select,
}

#[derive(Default)]
pub struct Tui {
    should_quit: bool,
    mode: Mode,
    terminal_size: Size,
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Bye!\r\n");
        }
    }
}

impl Tui {
    fn handle_resize_command(&mut self, size: Size) {
        // TODO: Handle resize
        self.terminal_size = size;
    }

    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;

        let mut tui = Self::default(); // mut
        let size = Terminal::size().unwrap_or_default();

        tui.handle_resize_command(size);

        //editor.update_message("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");
        //editor.refresh_status();

        Terminal::set_title("systemctl-manager")?;

        Ok(tui)
    }

    // fn handle_quit_command
    // fn process_command_during_search
    // fn process_command_during_select
    // fn process_command
    // fn evalueate_event
    // fn refresh_screen

    #[allow(clippy::print_stdout)]
    pub fn run(&mut self) {
        let _ = Terminal::move_caret_to(Position { col: 0, row: 0 });
        loop {
            if self.should_quit {
                break;
            }

            if let Ok(Event::Key(event)) = read()
                && event.code == Char('q')
            {
                self.should_quit = true;
            } else {
                let _ = Terminal::print("Hello, world!\r\n");
            }
        }
    }
}
