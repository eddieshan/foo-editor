use std::convert::{TryFrom};
use std::io;
use std::io::{Stdin, Stdout, Result, Error, Read, Write};

use crate::{ansi, text, theme};
use crate::win::{term, bindings};
use crate::win::term::TermState;

pub struct Editor {
    term_state: TermState
}

impl Editor {

    pub fn new(term_state: TermState) -> Self {
        Editor { term_state: term_state }
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

    fn gutter(&mut self, stdout: &mut Stdout) -> Result<()> {

        stdout.write(ansi::SET_BG)?;
        stdout.write(theme::GUTTER_BACKGROUND)?;
        stdout.write(ansi::SET_FG)?;
        stdout.write(theme::GUTTER_FOREGROUND)?;

        let height = self.term_state.buffer.dwMaximumWindowSize.Y;

        for i in 1..height {

            if i == height - 1 {
                stdout.write(ansi::UNDERLINE)?;
                print!("{:>3} ", i);
                stdout.write(ansi::RESET)?;

                stdout.write(ansi::SET_FG)?;
                stdout.write(theme::GUTTER_FOREGROUND)?;
                stdout.write(ansi::UNDERLINE)?;

                for i in 5..=self.term_state.buffer.dwMaximumWindowSize.X {
                    stdout.write(text::WHITESPACE)?;
                }
            } else {
                print!("{:>3} ", i);
            }

            stdout.write(ansi::NEXT_LINE)?;
        }

        stdout.write(ansi::RESET)?;
    
        Ok(())
    }

    fn status_bar(&self, stdout: &mut Stdout) -> Result<()> {

        stdout.write(ansi::SET_FG)?;
        stdout.write(theme::STATUS_FOREGROUND)?;

        let status = format!("BUFFER [{}, {}] | SCREEN [{}, {}]", 
                        self.term_state.buffer.dwSize.X, self.term_state.buffer.dwSize.Y, 
                        self.term_state.buffer.dwMaximumWindowSize.X, self.term_state.buffer.dwMaximumWindowSize.Y);

        let last_row = u16::try_from(self.term_state.buffer.dwMaximumWindowSize.Y).unwrap();
        let last_col = u16::try_from(self.term_state.buffer.dwMaximumWindowSize.X).unwrap();

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

        self.gutter(&mut stdout);
        self.status_bar(&mut stdout);
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
        self.term_state.restore();
    }
}