mod core;
mod text;
mod config;

mod term;
mod views;

mod buffers;
mod editor;

mod models;
mod controllers;

use crate::editor::*;
use crate::core::errors;

fn main() -> Result<(), errors::EditorError> {

    let state = term::configure()?;
    let mut editor = Editor::new(&state);
    
    editor.run()
}