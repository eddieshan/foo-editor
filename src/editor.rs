use std::io;
use std::io::{Result, Read, Write};

use crate::core::*;
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

        let mut cursor = Position { x: 1, y: 1 };

        gutter::render(&mut stdout, &cursor, &term_info);
        status_bar::render(&mut stdout, &cursor, &term_info);
        stdout.write(theme::TEXT_DEFAULT)?;
        stdout.write(theme::HOME)?;

        stdout.flush()?;

        let mut buffer: CharBuffer = [0; 4];

        let limit = Position { 
            x: term_info.screen_size.width - theme::GUTTER_WIDTH + 1, 
            y: term_info.screen_size.height
        };

        loop {
            buffer[0] = 0;
            buffer[1] = 0;
            buffer[2] = 0;
            buffer[3] = 0;
    
            let length = stdin.read(&mut buffer)?;
            let code = u32::from_be_bytes(buffer); // Conversion has to be big endian to match the input sequence.

            //print!("READ: ({}, {}, {}, {})", buffer[0], buffer[1], buffer[2], buffer[3]);
            let (result, next_cursor) = match code {
                keys::CTRL_Q => { break; },
                keys::CR     => (theme::LINE_FEED,   cursor.crlf()),
                keys::UP     => (&buffer[0..length], cursor.up()),
                keys::DOWN   => (&buffer[0..length], cursor.down()),
                keys::RIGHT  => (&buffer[0..length], cursor.right()),
                keys::LEFT   => (&buffer[0..length], cursor.left()),
                keys::BS     => (ansi::BACKDEL_1,    cursor.left()),
                ansi::DEL    => (ansi::DEL_1,        Position { x: cursor.x, y: cursor.y }),
                _            => (&buffer[0..length], cursor.right())
            };

            if next_cursor.is_valid(&limit) {
                cursor.x = next_cursor.x;
                cursor.y = next_cursor.y;
                stdout.write(result)?;
            }

            stdout.write(ansi::SAVE_CURSOR)?;
            
            let term_info = self.term.info()?;

            gutter::render(&mut stdout, &cursor, &term_info)?;
            status_bar::render(&mut stdout, &cursor, &term_info);

            stdout.write(ansi::RESTORE_CURSOR)?;
    
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