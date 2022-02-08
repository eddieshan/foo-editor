mod core;
mod text;
mod config;

mod term;
mod views;

mod buffers;
mod editor;

mod models;
mod controllers;

use crate::core::errors;

fn main() -> Result<(), errors::EditorError> {
    let term = term::configure()?;
    editor::run(&term)
}