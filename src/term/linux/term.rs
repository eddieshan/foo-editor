use std::io::{Result, Error, ErrorKind};
use std::convert::TryFrom;

use crate::core::*;
use crate::term::Term;
use super::bindings::*;

impl TryFrom<winsize> for Size {
    type Error = Error;
    fn try_from(coord: winsize) -> std::result::Result<Self, Self::Error> {
        match (usize::try_from(coord.ws_col), usize::try_from(coord.ws_row)) {
            (Ok(w), Ok(h)) => Ok(Size { width: w, height: h }),
            _              => Err(Error::last_os_error())
        }
    } 
}

pub struct LinuxTerm {
    state: termios
}

mod flags {

    use crate::term::linux::bindings::{tcflag_t, cc_t,
        BRKINT, ICRNL, INPCK, ISTRIP, IXON,
        OPOST, CS8, ECHO, ICANON, IEXTEN, ISIG
    };    

    pub const RAW_INPUT: tcflag_t = !(BRKINT | ICRNL | INPCK | ISTRIP | IXON);
    // Output mode flags: disable post processing.
    pub const NO_POST_PROCESSING: tcflag_t = !OPOST;
    // Control mode flags: set 8 bit chars.
    pub const CONTROL_MODE_8_BIT: tcflag_t = CS8;
    // Local mode flags: echo off, canonical off, no extended functions, no signal chars (^Z, ^C).
    pub const RAW_LOCAL: tcflag_t = !(ECHO | ICANON | IEXTEN | ISIG);
    // Control characters: no minimum number of bytes.
    pub const MIN_BYTES: cc_t = 0;
    // Control characters: 100ms timout.
    pub const TIMEOUT: cc_t = 1;
}

impl termios {
    pub fn raw_from(current: &termios) -> Self {
        let mut raw = termios {
            c_iflag: current.c_iflag & flags::RAW_INPUT,
            c_oflag: current.c_oflag & flags::NO_POST_PROCESSING,
            c_cflag: current.c_cflag | flags::CONTROL_MODE_8_BIT,
            c_lflag: current.c_lflag & flags::RAW_LOCAL,
            c_cc: current.c_cc.clone(),
            ..
            *current
        };
    
        raw.c_cc[VMIN] = flags::MIN_BYTES;
        raw.c_cc[VTIME] = flags::TIMEOUT;
    
        raw
    }    
}

fn get_window_size() -> Result<Size> {
    let mut ws = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0
    };

    unsafe {
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws) == -1 || ws.ws_col == 0 {
            Err(Error::new(ErrorKind::Other, ""))
        } else {
            Size::try_from(ws)
        }    
    }
}

impl Term for LinuxTerm {
    fn restore(&self) {
        unsafe {
            tcsetattr(STDIN_FILENO, TCSAFLUSH, &self.state);
        }
    }

    fn info(&self) -> Result<TermInfo> {

        let window_size = get_window_size()?;

        Ok(TermInfo {
            buffer_size: window_size.clone(),
            screen_size: window_size.clone(),
            cursor: Position { x: 0, y: 0}
        })
    }
}

pub fn configure() -> Result<LinuxTerm> {

    unsafe {
        let mut initial_state = termios {
            c_iflag: 0,
            c_oflag: 0,
            c_cflag: 0,
            c_lflag: 0,
            c_line: 0,
            c_cc: [0; 32],
            c_ispeed: 0,
            c_ospeed: 0        
        };

        if tcgetattr(STDIN_FILENO, &mut initial_state) < 0 {
            return Err(Error::new(ErrorKind::Other, ""));
        }
        
        let raw_term_state = termios::raw_from(&initial_state);
    
        // Flush terminal and set raw mode.
        if tcsetattr(STDIN_FILENO, TCSAFLUSH, &raw_term_state) < 0 {
            return Err(Error::new(ErrorKind::Other, ""));
        }

        Ok(LinuxTerm {
            state: initial_state
        })
    }
}