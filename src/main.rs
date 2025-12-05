#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
mod prelude;
mod tui;

use tui::Tui;

fn main() {
    Tui::new().unwrap().run();
}
