use crossterm::event::{
    KeyCode::{Char, Down, PageDown, PageUp, Up},
    KeyEvent, KeyModifiers,
};
use std::convert::TryFrom;

#[derive(Clone, Copy)]
pub enum Move {
    PageUp,
    PageDown,
    Up,
    Down,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;

    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = event;

        if modifiers == KeyModifiers::NONE {
            match code {
                PageUp => Ok(Self::PageUp),
                PageDown => Ok(Self::PageDown),
                Char('j') | Down => Ok(Self::Down),
                Char('k') | Up => Ok(Self::Up),
                _ => Err(format!("Unsupported code: {code:?}")),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}
