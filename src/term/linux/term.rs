use crate::core::geometry::*;
use crate::term::common::*;
use super::bindings::*;

impl From<winsize> for Size {
    fn from(coord: winsize) -> Self {
        Size { 
            width: usize::from(coord.ws_col), 
            height: usize::from(coord.ws_row)
        }
    }
}

// LinuxTerm only contains a backup of the termios state before
// setting the console in raw mode. termios could have been used
// directly to avoid an extra level of indirection but this is 
// cleaner and allows better separation from the OS bindings.
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

fn get_window_size() -> Result<Size, TermError> {
    let ws = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0
    };

    unsafe {
        let result = ioctl(STDOUT_FILENO, TIOCGWINSZ, &ws);
        match (result, ws.ws_col) {
            (-1, _) => Err(TermError::CannotGetTermAttributes),
            (_, 0)  => Err(TermError::InvalidTermAttributes),
            _       => Ok(Size::from(ws))
        }
    }
}

impl Term for LinuxTerm {
    fn restore(&self) -> Result<(), TermError> {
        set_term_attr(&self.state)?;
        Ok(())
    }

    fn info(&self) -> Result<TermInfo, TermError> {

        let window_size = get_window_size()?;

        // TODO: for now, for Linux, it is assumed that buffer size will be the 
        // same as window size. In Windows they are different values, retrieved
        // with separate sys calls. Consider simplifying this and use only the 
        // window size.
        Ok(TermInfo {
            buffer_size: window_size.clone(),
            screen_size: window_size.clone()
        })
    }
}

fn get_term_attr() -> Result<termios, TermError> {

    let mut state = termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; N_CONTROL_CHAR],
        c_ispeed: 0,
        c_ospeed: 0
    };

    unsafe {
        if tcgetattr(STDIN_FILENO, &mut state) < 0 {
            Err(TermError::CannotGetTermAttributes)
        } else {
            Ok(state)
        }
    }
}

fn set_term_attr(state: &termios) -> Result<(), TermError> {
    unsafe {
        if tcsetattr(STDIN_FILENO, TCSAFLUSH, state) < 0 {
            Err(TermError::CannotSetTermAttributes)
        } else {
            Ok(())
        }
    }
}

pub fn os_configure() -> Result<impl Term, TermError> {

    let initial_term_state = get_term_attr()?;
    let raw_term_state = termios::raw_from(&initial_term_state);
    set_term_attr(&raw_term_state)?;

    Ok(LinuxTerm {
        state: initial_term_state
    })
}