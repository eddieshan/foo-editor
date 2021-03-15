use std::convert::{TryFrom};
use std::io;
use std::io::{Stdout, Result, Read, Write};

use crate::{ansi, keys, theme};
use crate::win::term::{TermInfo, Term};

const MAX_LINE_WIDTH: usize = 200;
const WHITESPACE_LINE: [u8; MAX_LINE_WIDTH] = [keys::WHITESPACE; MAX_LINE_WIDTH];

type CharBuffer = [u8; 4];

pub struct Editor {
    term: Term
}

impl Editor {

    pub fn new(term: Term) -> Self {
        Editor { term: term }
    }

    fn set_pos(row: usize, col: usize, stdout: &mut Stdout) {
        stdout.write(ansi::SEQ);
        print!("{};{}", row, col);
        stdout.write(ansi::POS);
    }

    fn gutter(&mut self, stdout: &mut Stdout, info: &TermInfo) -> Result<()> {
        stdout.write(ansi::HOME)?;
        stdout.write(theme::GUTTER_DEFAULT)?;

        let height = info.screen_size.height;
        let cursor_y = info.cursor.y + 1;

        for i in 1..height {
            if i == cursor_y {
                stdout.write(theme::GUTTER_HIGHLIGHT)?;
                print!("{:>3} ", i);
                stdout.write(theme::GUTTER_DEFAULT)?;
            } else {
                print!("{:>3} ", i);
            }            
            
            stdout.write(ansi::NEXT_LINE)?;
        }

        stdout.write(ansi::RESET)?;
    
        Ok(())
    }

    fn status_bar(&self, stdout: &mut Stdout, info: &TermInfo) -> Result<()> {
        stdout.write(theme::STATUS_DEFAULT)?;

        let text_x = match info.cursor.x {
            0 => 1,
            x => x - theme::GUTTER_WIDTH + 1
        };
        let text_y = info.cursor.y + 1;

        let status = format!("{}:{},{}:{}", text_x, text_y, info.buffer_size.width, info.buffer_size.height);

        let last_col = info.screen_size.width + 1;
        let start_col = last_col - status.len();

        Editor::set_pos(info.screen_size.height, 0, stdout);
        stdout.write(&WHITESPACE_LINE[0..info.screen_size.width])?;
        Editor::set_pos(info.screen_size.height, start_col, stdout);
        print!("{}", status);

        stdout.write(ansi::RESET)?;
        
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {

        let mut stdout = io::stdout();
        let mut stdin = io::stdin();

        stdout.write(ansi::CLEAR)?;
        stdout.write(theme::HOME)?;
        stdout.flush()?;

        let term_info = self.term.info()?;

        self.gutter(&mut stdout, &term_info);
        self.status_bar(&mut stdout, &term_info);
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

            self.gutter(&mut stdout, &term_info)?;
            self.status_bar(&mut stdout, &term_info)?;

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