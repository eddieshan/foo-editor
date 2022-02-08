use std::io;
use std::io::{Read, Write};

use crate::core:: {
    errors::*,
    geometry::Position
};

use crate::text::keys::KeyBuffer;
use crate::text::keys;
use crate::models::editor::EditorState;
use crate::controllers::*;
use crate::config::theme;
use crate::buffers::gap_buffer::GapBuffer;
use crate::views::edit;
use crate::term::common::*;
use crate::term::vt100;

fn render(stdout: &mut impl Write, state: &EditorState) -> Result<(), EditorError> {
    stdout.write(vt100::CLEAR)?;
    stdout.write(theme::HOME)?;
    stdout.write(theme::TEXT_DEFAULT)?;

    edit::render(stdout, &state)?;

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

    render(&mut stdout, &state)?;
    
    let mut buffer: KeyBuffer = [0; 4];

    loop {
        buffer.fill(0);

        let length = stdin.read(&mut buffer)?;
        let code = u32::from_be_bytes(buffer); // Conversion has to be big endian to match the input sequence.

        match code {
            keys::CTRL_Q    => { break; },
            keys::CR        => state.buffer.insert(keys::LINE_FEED),
            keys::UP        => state.buffer.up(),
            keys::DOWN      => state.buffer.down(),
            keys::RIGHT     => state.buffer.right(),
            keys::LEFT      => state.buffer.left(),
            keys::HTAB      => { },
            keys::LN_START  => state.buffer.ln_start(),
            keys::LN_END    => state.buffer.ln_end(),
            keys::DEL       => state.buffer.del_right(),
            keys::BS        => state.buffer.del_left(),
            _               => {
                if length == 1 {
                    state.buffer.insert(buffer[0]);
                }
            }
        };

        render(&mut stdout, &state)?;
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