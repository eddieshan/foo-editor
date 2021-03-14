use std::convert::{TryFrom};
use std::io;
use std::io::{Stdout, Result, Read, Write};

use crate::{ansi, keys, theme};
use crate::win::term::{TermInfo, Term};

const MAX_LINE_WIDTH: usize = 200;
const WHITESPACE_LINE: [u8; MAX_LINE_WIDTH] = [keys::WHITESPACE; MAX_LINE_WIDTH];

type ReadBuffer = [u8; 3];

pub struct Editor {
    term: Term
}

impl Editor {

    pub fn new(term: Term) -> Self {
        Editor { term: term }
    }

    fn set_col(col: i16, stdout: &mut Stdout) {
        stdout.write(ansi::SEQ);
        print!("{}", col);
        stdout.write(ansi::COL);
    }

    fn set_pos(row: u16, col: u16, stdout: &mut Stdout) {
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
        stdout.write(ansi::SET_BG)?;
        stdout.write(theme::STATUS_BACKGROUND)?;
        stdout.write(ansi::SET_FG)?;
        stdout.write(theme::STATUS_FOREGROUND)?;

        let text_x = match info.cursor.x {
            0 => 1,
            x => x - theme::GUTTER_WIDTH + 1
        };
        let text_y = info.cursor.y + 1;

        let status = format!("{}:{},{}:{}", text_x, text_y, info.buffer_size.width, info.buffer_size.height);

        let last_row = info.screen_size.height;
        let last_col = info.screen_size.width + 1;
        let start_col = usize::try_from(last_col).unwrap() - status.len();

        Editor::set_pos(last_row, 0, stdout);
        stdout.write(&WHITESPACE_LINE[0..start_col - 1])?;

        print!("{}", status);

        stdout.write(ansi::RESET)?;
        
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {

        let mut stdout = io::stdout();
        let mut stdin = io::stdin();

        stdout.write(ansi::CLEAR)?;
        stdout.write(ansi::HOME)?;

        let term_info = self.term.info()?;

        self.gutter(&mut stdout, &term_info);
        self.status_bar(&mut stdout, &term_info);
        stdout.write(theme::HOME)?;

        stdout.flush()?;

        stdout.write(ansi::SET_FG)?;
        stdout.write(theme::TEXT_FOREGROUND)?;

        let mut buffer: ReadBuffer = [0, 0, 0];

        loop {
    
            let length = stdin.read(&mut buffer)?;

            match length {
                1 => match buffer[0] {
                    keys::CTRL_Q => { break; },
                    keys::CR     => { stdout.write(&theme::LINE_FEED)?; },
                    _            => { stdout.write(&buffer[0..1])?; }
                },
                _ => {
                    stdout.write(&buffer)?;
                }                
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