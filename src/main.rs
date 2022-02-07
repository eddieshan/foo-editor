mod core;
mod keys;
mod settings;
mod theme;

mod term;
mod components;

mod gap_buffer;
mod editor;

use crate::editor::*;

fn main() -> Result<(), EditorError> {

    let state = term::configure()?;
    let mut editor = Editor::new(&state);
    
    editor.run()
}