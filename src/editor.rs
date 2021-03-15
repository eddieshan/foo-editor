use std::io;
use std::io::{Stdout, Result, Read, Write};

use crate::{ansi, keys, theme};
use crate::components::{status_bar, gutter};
use crate::win::term::{TermInfo, Term};

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

        gutter::render(&mut stdout, &term_info);
        status_bar::render(&mut stdout, &term_info);
        stdout.write(theme::TEXT_DEFAULT)?;
        stdout.write(theme::HOME)?;

        stdout.flush()?;

        let mut buffer: CharBuffer = [0; 4];

        loop {
            buffer[0] = 0;
            buffer[1] = 0;
            buffer[2] = 0;
            buffer[3] = 0;
    
            let length = stdin.read(&mut buffer)?;
            let code = u32::from_be_bytes(buffer);

            //print!("READ: ({}, {}, {}, {})", buffer[0], buffer[1], buffer[2], buffer[3]);

            // TODO: for some reason char buffer conversion to u32 with from_ne_bytes results in wrong endiannes.
            // Explicit big endian works but perhaps depending on byte endianness here is not the right strategy.
            // To be reviewed.
            match code {
                keys::CTRL_Q => { break; },
                keys::CR     => { stdout.write(&theme::LINE_FEED)?; },
                keys::BS     => { stdout.write(&ansi::BACKDEL_1)?; },
                ansi::DEL    => { stdout.write(&ansi::DEL_1)?; },                
                _            => { stdout.write(&buffer[0..length])?; }
            }

            stdout.flush()?;

            stdout.write(ansi::SAVE_CURSOR)?;
            
            let term_info = self.term.info()?;

            gutter::render(&mut stdout, &term_info)?;
            status_bar::render(&mut stdout, &term_info);

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