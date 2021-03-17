use std::io;
use std::io::{Result, Read, Write};

use crate::{ansi, keys, theme};
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
        stdout.write(theme::HOME)?;
        stdout.flush()?;

        let term_info = self.term.info()?;
        let mut cursor = Cursor::new(&term_info.screen_size);

        let start_pos = cursor.pos();

        gutter::render(&mut stdout, &start_pos, &term_info);
        status_bar::render(&mut stdout, &start_pos, &term_info);

        stdout.write(theme::TEXT_DEFAULT)?;
        stdout.write(theme::HOME)?;

        stdout.flush()?;

        let mut buffer: CharBuffer = [0; 4];
        let empty: &[u8] = &[];

        loop {
            buffer[0] = 0;
            buffer[1] = 0;
            buffer[2] = 0;
            buffer[3] = 0;
    
            let length = stdin.read(&mut buffer)?;
            let code = u32::from_be_bytes(buffer); // Conversion has to be big endian to match the input sequence.

            //print!("READ: ({}, {}, {}, {})", buffer[0], buffer[1], buffer[2], buffer[3]);
            let (result, is_valid_cursor) = match code {
                keys::CTRL_Q    => { break; },
                keys::CR        => (empty, cursor.crlf()),
                keys::UP        => (empty, cursor.up()),
                keys::DOWN      => (empty, cursor.down()),
                keys::RIGHT     => (empty, cursor.right()),
                keys::LEFT      => (empty, cursor.left()),
                keys::HTAB      => (empty, cursor.htab()),
                keys::LN_START  => (empty, cursor.ln_start()),
                keys::LN_END    => (empty, cursor.ln_end()),
                keys::BS        => (ansi::BACKDEL_1,    cursor.left()),
                ansi::DEL       => (ansi::DEL_1,        true),
                _               => (&buffer[0..length], cursor.right())
            };

            if is_valid_cursor {
                stdout.write(result)?;
            }
            
            let pos = cursor.pos();

            gutter::render(&mut stdout, &pos, &term_info)?;
            status_bar::render(&mut stdout, &pos, &term_info);

            let screen_pos = cursor.screen_pos();
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