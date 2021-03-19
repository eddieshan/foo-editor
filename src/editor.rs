use std::io;
use std::io::{Result, Read, Write};

use crate::core::Position;
use crate::{ansi, keys, theme, settings};
use crate::gap_buffer::GapBuffer;
use crate::cursor::Cursor;
use crate::components::{status_bar, gutter};
use crate::win::term::Term;

type CharBuffer = [u8; 4];

pub struct Editor {
    term: Term
}

impl Editor {

    pub fn new(term: Term) -> Self {
        Editor { term: term }
    }

    pub fn run(&mut self) -> Result<()> {

        let mut stdout = io::stdout();
        let mut stdin = io::stdin();

        stdout.write(ansi::CLEAR)?;

        let term_info = self.term.info()?;
        let buffer_width = term_info.screen_size.width - settings::GUTTER_WIDTH;

        let start_pos = Position { x: 1, y: 1 };

        gutter::render(&mut stdout, start_pos.y, 1, &term_info);
        status_bar::render(&mut stdout, &start_pos, &term_info);

        stdout.write(theme::HOME)?;

        stdout.flush()?;
        
        let mut buffer: CharBuffer = [0; 4];
        let empty: &[u8] = &[];
        let mut gap_buffer = GapBuffer::new();

        loop {
            buffer[0] = 0;
            buffer[1] = 0;
            buffer[2] = 0;
            buffer[3] = 0;
   
            let length = stdin.read(&mut buffer)?;
            let code = u32::from_be_bytes(buffer); // Conversion has to be big endian to match the input sequence.

            match code {
                keys::CTRL_Q    => { break; },
                keys::CR        => gap_buffer.insert(10),
                keys::UP        => { },
                keys::DOWN      => { },
                keys::RIGHT     => gap_buffer.right(),
                keys::LEFT      => gap_buffer.left(),
                keys::HTAB      => { },
                keys::LN_START  => { },
                keys::LN_END    => { },
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
            
            let (total_ln, lncol) = gap_buffer.dump(&mut stdout);
            
            gutter::render(&mut stdout, lncol.y, total_ln, &term_info)?;
            status_bar::render(&mut stdout, &lncol, &term_info);

            let screen_pos = Position { x: lncol.x + settings::GUTTER_WIDTH, y: lncol.y };

            ansi::pos(screen_pos.y, screen_pos.x, &mut stdout);            
    
            stdout.flush()?;
        }

        Ok(())
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let mut stdout = io::stdout();
        stdout.write(ansi::RESET);
        stdout.write(ansi::CLEAR);
        stdout.flush();
        self.term.restore();
    }
}