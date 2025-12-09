use crossterm::event::{Event, KeyCode::Char, read};
use std::{
    io::Error,
    panic::{set_hook, take_hook},
};

mod annotatedstring;
mod annotation;
mod annotationtype;
mod terminal;
mod tuistatus;
mod uicomponents;

use crate::prelude::*;
use annotatedstring::AnnotatedString;
use annotation::Annotation;
use annotationtype::AnnotationType;
use terminal::Terminal;
use tuistatus::TuiStatus;
use uicomponents::{UIComponent, View};

#[derive(Eq, PartialEq, Default, Debug)]
enum Mode {
    Filter,
    Search,
    #[default]
    Normal,
}

#[derive(Default)]
pub struct Tui {
    should_quit: bool,
    mode: Mode,
    terminal_size: Size,
    view: View,
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
        self.terminal_size = size;

        self.view.resize(size);

        // self.view.resize(Size {
        //     height: size.height.saturating_sub(2),
        //     width: size.width,
        // });
        //
        // let bar_size = Size {
        //     height: 1,
        //     width: size.width,
        // };
        //
        // self.message_bar.resize(bar_size);
        // self.status_bar.resize(bar_size);
        // self.command_bar.resize(bar_size);
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
        tui.view.load()?;
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
    fn refresh_screen(&mut self) {
        if self.terminal_size.width == 0 || self.terminal_size.height == 0 {
            return;
        }

        let _ = Terminal::hide_caret();
        // let bottom_bar_row = self.terminal_size.height.saturating_sub(1);

        // if self.in_prompt() {
        //     self.command_bar.render(bottom_bar_row);
        // } else {
        //     self.message_bar.render(bottom_bar_row);
        // }

        // if self.terminal_size.height > 1 {
        //     self.status_bar
        //         .render(self.terminal_size.height.saturating_sub(2));
        // }

        // if self.terminal_size.height > 2 {
        self.view.render(0);
        // }

        let new_cursor_pos = self.view.cursor_position();

        debug_assert!(new_cursor_pos <= self.terminal_size.height,);

        // let _ = Terminal::move_caret_to(new_cursor_pos); // TODO: Handle this with modes
        let _ = Terminal::show_caret();
        let _ = Terminal::execute();
    }

    #[allow(clippy::print_stdout)]
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            if let Ok(Event::Key(event)) = read()
                && event.code == Char('q')
            {
                self.should_quit = true;
            }

            // match read() {
            //     Ok(event) => self.evaluate_event(event),
            //     Err(err) => {
            //         #[cfg(debug_assertions)]
            //         {
            //             panic!("Could not read event: {err:?}")
            //         }
            //
            //         #[cfg(not(debug_assertions))]
            //         {
            //             let _ = err;
            //         }
            //     }
            // }
            //
            // self.refresh_status();
        }
    }
}
