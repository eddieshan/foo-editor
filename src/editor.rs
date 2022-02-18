use std::io;
use std::io::Write;

use crate::core::errors::*;

use crate::text::{keys, keys::ReadKey};
use crate::config::theme;
use crate::term::{common::*, vt100};
use crate::models::editor::EditorState;
use crate::controllers::*;
use crate::views;

fn render<T: Write>(stdout: &mut T, view: View<T>, state: &EditorState) -> Result<(), EditorError> {
    stdout.write(vt100::CLEAR)?;
    stdout.write(theme::HOME)?;
    stdout.write(theme::TEXT_DEFAULT)?;
    view(stdout, state)?;
    stdout.flush()?;

    Ok(())
}

pub fn run(term: &impl Term) -> Result<(), EditorError> {

    let mut stdout = io::stdout();
    let mut stdin = io::stdin();

    let mut state = EditorState::new(term.window()?);

    let mut action_result = ActionResult {
        view: views::edit::render,
        controller: edit_controller::edit
    };

    render(&mut stdout, action_result.view, &state)?;
 
    while let Ok(key) = stdin.read_key() {
        action_result = match key.code {
            keys::CTRL_Q => { break; },
            _            => (action_result.controller)(&key, &mut state)?
        };    

        render(&mut stdout, action_result.view, &state)?;
    }

    reset(&mut stdout)?;
    term.restore()?;

    Ok(())
}

fn reset(stdout: &mut impl Write,) -> io::Result<()> {
    stdout.write(vt100::RESET)?;
    stdout.write(vt100::CLEAR)?;
    stdout.flush()
}