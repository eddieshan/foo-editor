use std::io;
use std::io::{Stdin, Stdout, Result, Error, Read, Write};

use crate::ansi;
use crate::windows::{term, bindings};
use crate::windows::term::TermState;

pub struct Editor {
    term_state: TermState
}

impl Editor {

    pub fn new(term_state: TermState) -> Self {
        Editor { term_state: term_state }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let mut stdout = io::stdout();
        stdout.write(ansi::CLEAR);
        stdout.flush();
        self.term_state.restore();
    }
}