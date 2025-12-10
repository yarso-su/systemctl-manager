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
mod operation;
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
use operation::{Operation, OperationType};
use terminal::Terminal;
use tuistatus::TuiStatus;
use uicomponents::{FilterBar, MessageBar, SearchBar, StatusBar, UIComponent, View};

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
    fn is_search(mode: Self) -> bool {
        mode == Self::Search
    }

    fn is_filter(mode: Self) -> bool {
        mode == Self::Filter
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
    search_bar: SearchBar,
    message_bar: MessageBar,
    operation: Option<Operation>,
    multiplier: Option<String>,
}

impl Drop for Tui {
    fn drop(&mut self) {
        let _ = Terminal::terminate();

        if self.should_quit
            && let Some(operation) = &self.operation
        {
            let _ = match operation.execute() {
                Ok(()) => Terminal::print("✓ Command executed successfully\n"),
                Err(e) => Terminal::print(format!("✗ Command failed: {e}\n").as_str()),
            };
        }
    }
}

impl Tui {
    fn append_multiplier(&mut self, digit: char) {
        if let Some(multiplier) = self.multiplier.as_mut() {
            multiplier.push(digit);
        } else {
            self.multiplier = Some(String::from(digit));
        }
    }

    fn get_multiplier(&self) -> Option<usize> {
        if let Some(multiplier) = self.multiplier.as_ref() {
            multiplier.parse().ok()
        } else {
            None
        }
    }

    fn set_operation_result(&mut self, operation_type: OperationType) {
        let name = self.view.get_selected_service_name();
        if let Some(name) = name {
            self.operation = Some(Operation::new(operation_type, name));
        }

        self.should_quit = true;
    }

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

        let bar_size = Size {
            height: 1,
            width: size.width,
        };

        self.status_bar.resize(bar_size);
        self.filter_bar.resize(bar_size);
        self.message_bar.resize(bar_size);
        self.search_bar.resize(bar_size);
    }

    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));

        Terminal::initialize()?;

        let mut tui = Self::default();
        let size = Terminal::size().unwrap_or_default();

        tui.view.set_hilight_selected_line(true);
        tui.handle_resize_command(size);
        tui.view.load()?;
        tui.refresh_status();

        Terminal::set_title("systemctl-manager")?;

        Ok(tui)
    }

    fn refresh_screen(&mut self) {
        if self.terminal_size.width == 0 || self.terminal_size.height == 0 {
            return;
        }

        let _ = Terminal::hide_caret();
        self.filter_bar.render(0);

        if self.terminal_size.height > 1 {
            self.status_bar
                .render(self.terminal_size.height.saturating_sub(2));
        }

        if self.terminal_size.height > 2 {
            if Mode::is_search(self.mode) {
                self.search_bar
                    .render(self.terminal_size.height.saturating_sub(1));
            } else {
                self.message_bar
                    .render(self.terminal_size.height.saturating_sub(1));
            }
        }

        if self.terminal_size.height > 3 {
            self.view.render(1);
        }

        if Mode::is_filter(self.mode) {
            let _ = Terminal::move_caret_to(0, Some(self.filter_bar.caret_position_col()));
            let _ = Terminal::show_caret();
        }

        if Mode::is_search(self.mode) {
            let _ = Terminal::move_caret_to(
                self.terminal_size.height.saturating_sub(1),
                Some(self.search_bar.caret_position_col()),
            );
            let _ = Terminal::show_caret();
        }

        let _ = Terminal::execute();
    }

    fn process_command_during_normal(&mut self, command: Command) {
        if matches!(command, System(Quit)) {
            self.should_quit = true;
            return;
        }

        if let Edit(Insert('0'..='9')) = command
            && let Edit(Insert(ch)) = command
        {
            self.append_multiplier(ch);

            if let Some(multiplier) = &self.multiplier {
                self.message_bar.update_message(multiplier);
            }

            return;
        }

        match command {
            Edit(Insert('/')) => {
                self.mode = Mode::Search;
                self.view.enter_search();
                self.search_bar.redraw();
            }
            Edit(Insert('i' | 'I' | 'a' | 'A')) => {
                self.mode = Mode::Filter;

                self.view.set_hilight_selected_line(false);
                self.view.scroll_to_start();
                self.message_bar.clear_message();
            }
            Edit(Insert('z')) => {
                self.message_bar.update_message("Filter mode: i/a/I/A | Search mode: / | Dismiss: ctrl+c/esc | Confirm: enter | Search next: n | Search prev: N");
            }
            Edit(Insert('n')) => {
                self.view.search_next();
            }
            Edit(Insert('N')) => {
                self.view.search_prev();
            }
            Edit(Insert('w')) => {
                self.set_operation_result(OperationType::Start);
            }
            Edit(Insert('e')) => {
                self.set_operation_result(OperationType::Stop);
            }
            Edit(Insert('r')) => {
                self.set_operation_result(OperationType::Reload);
            }
            Edit(Insert('t')) => {
                self.set_operation_result(OperationType::Restart);
            }
            Edit(Insert('y')) => {
                self.set_operation_result(OperationType::Enable);
            }
            Edit(Insert('u')) => {
                self.set_operation_result(OperationType::Disable);
            }
            Edit(Insert('j')) => {
                let multiplier = self.get_multiplier();
                self.view.handle_move_command(Down, multiplier);

                if multiplier.is_some() {
                    self.message_bar.clear_message();
                }
            }
            Edit(Insert('k')) => {
                let multiplier = self.get_multiplier();
                self.view.handle_move_command(Up, multiplier);

                if multiplier.is_some() {
                    self.message_bar.clear_message();
                }
            }
            Move(move_command) => {
                let multiplier = self.get_multiplier();
                self.view.handle_move_command(move_command, multiplier);

                if multiplier.is_some() {
                    self.message_bar.clear_message();
                }
            }
            _ => {}
        }

        self.message_bar.redraw();
        self.multiplier.take();
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

    fn process_command_during_search(&mut self, command: Command) {
        match command {
            System(Dismiss) => {
                self.mode = Mode::Normal;
                self.view.dismiss_search();
                self.message_bar.redraw();
                self.search_bar.clear_value();
            }
            Edit(InsertNewLine) => {
                self.mode = Mode::Normal;
                self.view.exit_search();
                self.message_bar.redraw();
            }
            Edit(command) => {
                self.search_bar.handle_edit_command(command);
                self.view.search(&self.search_bar.value());
            }
            Move(Down) => {
                self.view.search_next();
            }
            Move(Up) => {
                self.view.search_prev();
            }
            _ => {}
        }
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
