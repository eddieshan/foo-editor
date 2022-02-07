mod core;
mod keys;
mod config;

mod term;
mod components;

mod buffers;
mod editor;

use crate::editor::*;

fn main() -> Result<(), EditorError> {

    let state = term::configure()?;
    let mut editor = Editor::new(&state);
    
    editor.run()
}