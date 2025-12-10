use crossterm::event::{
    KeyCode::{self, Char},
    KeyEvent, KeyModifiers,
};
use std::convert::TryFrom;

use crate::prelude::*;

#[derive(Clone, Copy)]
pub enum System {
    Save,
    Resize(Size),
    Quit,
    Dismiss,
    Search,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::CONTROL {
            match code {
                Char('s') => Ok(Self::Save),
                Char('q') => Ok(Self::Quit),
                Char('f') => Ok(Self::Search),
                Char('c') => Ok(Self::Dismiss),
                _ => Err(format!("Unsupported CONTROL+{code:?} combination")),
            }
        } else if modifiers == KeyModifiers::NONE && matches!(code, KeyCode::Esc) {
            Ok(Self::Dismiss)
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
