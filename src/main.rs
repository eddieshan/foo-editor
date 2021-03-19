mod core;
mod ansi;
mod keys;
mod settings;
mod theme;
mod cursor;

mod win {
    pub mod bindings;
    pub mod term;
}

mod components {
    pub mod status_bar;
    pub mod gutter;
}

mod gap_buffer;
mod editor;

use std::io::Result;

use crate::win::term;
use crate::editor::Editor;

fn main() -> Result<()> {

    let state = term::configure()?;
    let mut editor = Editor::new(state);
    
    editor.run()
}