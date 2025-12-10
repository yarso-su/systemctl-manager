use crossterm::event::{Event, KeyEvent, KeyEventKind, read};
use std::{
    fmt::Display,
    io::Error,
    panic::{set_hook, take_hook},
};

mod annotatedstring;
mod annotation;
mod annotationtype;
mod command;
mod terminal;
mod tuistatus;
mod uicomponents;

use crate::prelude::*;
use annotatedstring::AnnotatedString;
use annotation::Annotation;
use annotationtype::AnnotationType;
use command::{
    Command::{self, Edit, Move, System},
    Edit::{Insert, InsertNewLine},
    Move::{Down, Up},
    System::{Dismiss, Quit, Resize},
};
use terminal::Terminal;
use tuistatus::TuiStatus;
use uicomponents::{FilterBar, MessageBar, StatusBar, UIComponent, View};

#[derive(Eq, PartialEq, Default, Clone, Copy)]
pub enum Mode {
    Filter,
    Search,
    #[default]
    Normal,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Filter => write!(f, "FILTER"),
            Self::Search => write!(f, "SEARCH"),
            Self::Normal => write!(f, "NORMAL"),
        }
    }
}

impl Mode {
    fn is_normal(&self) -> bool {
        *self == Self::Normal
    }

    fn is_search(&self) -> bool {
        *self == Self::Search
    }

    fn is_filter(&self) -> bool {
        *self == Self::Filter
    }
}

#[derive(Default)]
pub struct Tui {
    should_quit: bool,
    mode: Mode,
    terminal_size: Size,
    view: View,
    status_bar: StatusBar,
    filter_bar: FilterBar,
    message_bar: MessageBar,
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
    fn refresh_status(&mut self) {
        self.status_bar
            .update_status(self.view.get_status(self.mode));
    }

    fn handle_resize_command(&mut self, size: Size) {
        self.terminal_size = size;

        self.view.resize(Size {
            height: size.height.saturating_sub(3),
            width: size.width,
        });
        //
        let bar_size = Size {
            height: 1,
            width: size.width,
        };

        self.status_bar.resize(bar_size);
        self.filter_bar.resize(bar_size);
        self.message_bar.resize(bar_size);
        // self.message_bar.resize(bar_size);
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

        tui.view.set_hilight_selected_line(true);
        tui.handle_resize_command(size);
        tui.view.load()?;
        //editor.update_message("HELP: Ctrl-F = find | Ctrl-S = save | Ctrl-Q = quit");
        tui.refresh_status();

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
        self.filter_bar.render(0);

        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            self.message_bar
                .render(self.terminal_size.height.saturating_sub(1));
        }

        if self.terminal_size.height > 3 {
            self.view.render(1);
        }

        // let new_cursor_pos = self.view.cursor_position();
        //
        // debug_assert!(new_cursor_pos <= self.terminal_size.height,);

        // let _ = Terminal::move_caret_to(new_cursor_pos); // TODO: Handle this with modes
        if self.mode.is_filter() {
            let _ = Terminal::move_caret_to(0, Some(self.filter_bar.caret_position_col()));
            let _ = Terminal::show_caret();
        }

        let _ = Terminal::execute();
    }

    fn process_command_during_normal(&mut self, command: Command) {
        if matches!(command, System(Quit)) {
            self.should_quit = true;
            return;
        }

        match command {
            Edit(Insert('/')) => {
                self.mode = Mode::Search;
                // enter search mode
            }
            Edit(Insert('i' | 'I' | 'a' | 'A')) => {
                self.mode = Mode::Filter;

                self.view.set_hilight_selected_line(false);
                self.view.scroll_to_start();
                // enter filter mode
            }
            Edit(Insert('j')) => {
                self.view.handle_move_command(Down);
            }
            Edit(Insert('k')) => {
                self.view.handle_move_command(Up);
            }
            Move(move_command) => {
                self.view.handle_move_command(move_command);
            }
            _ => {}
        }

        // match command {
        //     // System(Quit | Resize(_) | Dismiss) => {}
        //     // System(Search) => self.set_prompt(PromptType::Search),
        //     // System(Save) => self.handle_save_command(),
        //     // Edit(InsertNewLine) => {}
        //     Move(move_command) => self.view.handle_move_command(move_command),
        //     _ => {}
        // }
        // TODO
    }

    fn process_command_during_filter(&mut self, command: Command) {
        match command {
            System(Dismiss) | Edit(InsertNewLine) => {
                self.mode = Mode::Normal;

                self.view.set_hilight_selected_line(true);
            }
            Edit(command) => {
                self.filter_bar.handle_edit_command(command);
                self.view.filter(&self.filter_bar.value());
            }
            _ => {}
        }
    }

    fn process_command_during_search(&self, _command: Command) {
        // TODO: Implement this
    }

    fn process_command(&mut self, command: Command) {
        if let System(Resize(size)) = command {
            self.handle_resize_command(size);
            return;
        }

        match self.mode {
            Mode::Normal => self.process_command_during_normal(command),
            Mode::Filter => self.process_command_during_filter(command),
            Mode::Search => self.process_command_during_search(command),
        }
    }

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };

        if should_process && let Ok(command) = Command::try_from(event) {
            self.process_command(command);
        }
    }

    #[allow(clippy::print_stdout)]
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();

            if self.should_quit {
                break;
            }

            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}")
                    }

                    #[cfg(not(debug_assertions))]
                    {
                        let _ = err;
                    }
                }
            }

            self.refresh_status();
        }
    }
}
