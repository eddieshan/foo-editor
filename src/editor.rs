use std::io;
use std::io::{Result, Read, Write};

use crate::core::Position;
use crate::{ansi, keys, theme, settings};
use crate::gap_buffer::GapBuffer;
use crate::components::{status_bar, gutter};
use crate::term::Term;

type CharBuffer = [u8; 4];

pub struct Editor<'a> {
    term: &'a (dyn Term + 'a)
}

impl<'a> Editor<'a> {

    pub fn new(term: &'a impl Term) -> Self {
        Editor { term: term }
    }

    pub fn run(&mut self) -> Result<()> {

        let mut stdout = io::stdout();
        let mut stdin = io::stdin();

        stdout.write(ansi::CLEAR)?;

        let term_info = self.term.info()?;

        let start_pos = Position { x: 1, y: 1 };

        gutter::render(&mut stdout, start_pos.y, 1)?;
        status_bar::render(&mut stdout, &start_pos, &term_info)?;

        stdout.write(theme::HOME)?;

        stdout.flush()?;
        
        let mut buffer: CharBuffer = [0; 4];
        let mut gap_buffer = GapBuffer::new();

        loop {
            buffer.fill(0);
   
            let length = stdin.read(&mut buffer)?;
            let code = u32::from_be_bytes(buffer); // Conversion has to be big endian to match the input sequence.

            match code {
                keys::CTRL_Q    => { break; },
                keys::CR        => gap_buffer.insert(keys::LINE_FEED),
                keys::UP        => gap_buffer.up(),
                keys::DOWN      => gap_buffer.down(),
                keys::RIGHT     => gap_buffer.right(),
                keys::LEFT      => gap_buffer.left(),
                keys::HTAB      => { },
                keys::LN_START  => gap_buffer.ln_start(),
                keys::LN_END    => gap_buffer.ln_end(),
                ansi::DEL       => gap_buffer.del_right(),
                keys::BS        => gap_buffer.del_left(),
                _               => {
                    if length == 1 {
                        gap_buffer.insert(buffer[0]);
                    }
                }
            };

            stdout.write(ansi::CLEAR)?;
            stdout.write(theme::HOME)?;
            stdout.write(theme::TEXT_DEFAULT)?;
            
            let (total_ln, lncol) = gap_buffer.dump(&mut stdout)?;
            
            gutter::render(&mut stdout, lncol.y, total_ln)?;
            status_bar::render(&mut stdout, &lncol, &term_info)?;

            let screen_pos = Position { x: lncol.x + settings::GUTTER_WIDTH, y: lncol.y };

            ansi::pos(screen_pos.y, screen_pos.x, &mut stdout)?;
    
            stdout.flush()?;
        }

        Ok(())
    }
}

fn reset() -> Result<()> {
    let mut stdout = io::stdout();
    stdout.write(ansi::RESET)?;
    stdout.write(ansi::CLEAR)?;
    stdout.flush()
}

impl<'a> Drop for Editor<'a> {
    fn drop(&mut self) {
        // TODO: how to handle errors properly in destructor?
        // Does it make sense to log errors in reset or restore?
        // Since a Result cannot be returned in Drop, is it better to 
        // restore state in another place that allows error propagation.
        reset().and_then(|()| self.term.restore());
    }
}