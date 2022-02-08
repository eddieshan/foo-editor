use std::io;
use std::io::{Read, Write};

use crate::core::errors::*;

use crate::text::keys::KeyBuffer;
use crate::text::keys;
use crate::models::editor::EditorState;
use crate::views;
use crate::controllers;
use crate::controllers::*;
use crate::config::theme;
use crate::buffers::gap_buffer::GapBuffer;
use crate::term::common::*;
use crate::term::vt100;

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

    let mut state = EditorState {
        term_info: term.info()?,
        buffer: GapBuffer::new()
    };

    let mut action_result = ActionResult {
        view: views::edit::render,
        controller: controllers::edit_controller::edit
    };

    render(&mut stdout, action_result.view, &state)?;
    
    let mut buffer: KeyBuffer = [0; 4];

    loop {
        buffer.fill(0);

        let length = stdin.read(&mut buffer)?;
        let code = u32::from_be_bytes(buffer); // Conversion has to be big endian to match the input sequence.

        action_result = match code {
            keys::CTRL_Q => { break; },
            _            => (action_result.controller)(&buffer, length, &mut state)?
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