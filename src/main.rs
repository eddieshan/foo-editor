mod win {
    pub mod bindings;
    pub mod term;
}

mod ansi;
mod text;
mod theme;
mod editor;

use std::io::{Result, Error};

use crate::win::term;
use crate::editor::Editor;

fn main() -> Result<()> {
    let state = term::configure()?;

    let mut editor = Editor::new(state);

    editor.run();

    Ok(())
}