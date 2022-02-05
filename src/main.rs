mod core;
mod ansi;
mod keys;
mod settings;
mod theme;

mod term;
mod components;

mod gap_buffer;
mod editor;

use std::io::Result;

use crate::editor::Editor;

fn main() -> Result<()> {

    let state = term::configure()?;
    let mut editor = Editor::new(&state);
    
    editor.run()
}