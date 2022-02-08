use std::io;
use std::io::{Read, Write};

use crate::core:: {
    errors::*,
    geometry::Position
};

use crate::text::keys::KeyBuffer;
use crate::text::keys;
use crate::models::editor::EditorState;
use crate::config::theme;
use crate::buffers::gap_buffer::GapBuffer;
use crate::views::edit;
use crate::term::common::*;
use crate::term::vt100;

pub struct Editor<'a> {
    term: &'a (dyn Term + 'a)
}

impl<'a> Editor<'a> {

    pub fn new(term: &'a impl Term) -> Self {
        Editor { term: term }
    }

    pub fn run(&mut self) -> Result<(), EditorError> {

        let mut stdout = io::stdout();
        let mut stdin = io::stdin();

        stdout.write(vt100::CLEAR)?;

        let mut state = EditorState {
            term_info: self.term.info()?,
            buffer: GapBuffer::new()
        };

        edit::render(&mut stdout, &state);

        stdout.write(theme::HOME)?;

        stdout.flush()?;
        
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

            stdout.write(vt100::CLEAR)?;
            stdout.write(theme::HOME)?;
            stdout.write(theme::TEXT_DEFAULT)?;

            edit::render(&mut stdout, &state);
    
            stdout.flush()?;
        }

        Ok(())
    }
}

fn reset() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write(vt100::RESET)?;
    stdout.write(vt100::CLEAR)?;
    stdout.flush()
}

impl<'a> Drop for Editor<'a> {
    fn drop(&mut self) {
        // TODO: how to handle errors properly in destructor?
        // Does it make sense to log errors in reset or restore?
        // Since a Result cannot be returned in Drop, is it better to 
        // restore state in another place that allows error propagation.
        let _ = reset().map(|()| self.term.restore());
    }
}