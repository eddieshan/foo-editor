use std::convert::{TryFrom};
use std::io;
use std::io::{Stdout, Result, Read, Write};

use crate::{ansi, text, theme};
use crate::win::term::{TermInfo, Term};

const MAX_LINE_WIDTH: usize = 200;
const WHITESPACE: u8 = 0x20;
const WHITESPACE_LINE: [u8; MAX_LINE_WIDTH] = [WHITESPACE; MAX_LINE_WIDTH];

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

        stdout.write(ansi::SET_BG)?;
        stdout.write(theme::GUTTER_BACKGROUND)?;
        stdout.write(ansi::SET_FG)?;
        stdout.write(theme::GUTTER_FOREGROUND)?;

        let height = info.screen_size.height;

        for i in 1..height {

            if i == height - 1 {
                stdout.write(ansi::UNDERLINE)?;
                print!("{:>3} ", i);
                stdout.write(ansi::RESET)?;

                stdout.write(ansi::SET_FG)?;
                stdout.write(theme::GUTTER_FOREGROUND)?;
                stdout.write(ansi::UNDERLINE)?;

                stdout.write(&WHITESPACE_LINE[5..usize::try_from(info.screen_size.width).unwrap()])?;
            } else {
                print!("{:>3} ", i);
            }

            stdout.write(ansi::NEXT_LINE)?;
        }

        stdout.write(ansi::RESET)?;
    
        Ok(())
    }

    fn status_bar(&self, stdout: &mut Stdout, info: &TermInfo) -> Result<()> {

        stdout.write(ansi::SET_FG)?;
        stdout.write(theme::STATUS_FOREGROUND)?;

        let status = format!("BUFFER [{}, {}] | SCREEN [{}, {}]", 
                        info.buffer_size.width, info.buffer_size.height, 
                        info.screen_size.width, info.screen_size.height);

        let last_row = info.screen_size.height;
        let last_col = info.screen_size.width;

        let start_col = last_col - u16::try_from(status.len()).unwrap();

        Editor::set_pos(last_row, start_col, stdout);

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

        let mut buffer: [u8; 3] = [0, 0, 0];

        loop {
    
            let length = stdin.read(&mut buffer)?;
    
            match (length, buffer[0] as char) {
                (1, 'q') => {
                    break;
                },
                (1, c) => {
                    print!("{}", c);
                },
                _ => {
                    stdout.write(&buffer)?;
                }
            }
    
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