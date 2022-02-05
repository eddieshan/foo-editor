#![allow(non_camel_case_types, non_snake_case)]
use std::os::raw::*;

pub type cc_t = c_uchar;
pub type speed_t = c_uint;
pub type tcflag_t = c_uint;

pub const N_CONTROL_CHAR: usize = 32;
pub const STDIN_FILENO: c_int = 0;
pub const STDOUT_FILENO: c_int	= 1;
pub const TIOCGWINSZ: c_ulong = 0x5413;
pub const TCSAFLUSH: c_int = 2;

#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; N_CONTROL_CHAR],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

#[repr(C)]
pub struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

// Input mode flags.
//pub const IGNBRK:  tcflag_t = 0o000001;
pub const BRKINT:  tcflag_t = 0o000002;
//pub const IGNPAR:  tcflag_t = 0o000004;
//pub const PARMRK:  tcflag_t = 0o000010;
pub const INPCK:   tcflag_t = 0o000020;
pub const ISTRIP:  tcflag_t = 0o000040;
//pub const INLCR:   tcflag_t = 0o000100;
//pub const IGNCR:   tcflag_t = 0o000200;
pub const ICRNL:   tcflag_t = 0o000400;
//pub const IUCLC:   tcflag_t = 0o001000;
pub const IXON:    tcflag_t = 0o002000;
//pub const IXANY:   tcflag_t = 0o004000;
//pub const IXOFF:   tcflag_t = 0o010000;
//pub const IMAXBEL: tcflag_t = 0o020000;
//pub const IUTF8:   tcflag_t = 0o040000;

// Output mode flags.
pub const OPOST:   tcflag_t = 0o000001;

// Control mode flags.
pub const CS8:      tcflag_t = 0o000060;

// Local mode flags.
pub const ISIG:    tcflag_t = 0o000001;
pub const ICANON:  tcflag_t = 0o000002;
pub const ECHO:    tcflag_t = 0o000010;
pub const IEXTEN:  tcflag_t = 0o100000;

// Control characters.
pub const VTIME:    usize = 5;
pub const VMIN:     usize = 6;

#[link(name = "c")]
extern "C" {
    pub fn tcgetattr(fd: c_int, termios_p: *mut termios) -> c_int;
    pub fn tcsetattr(fd: c_int, optional_actions: c_int, termios_p: *const termios) -> c_int;
    pub fn ioctl(fd: c_int, req: c_ulong, ...) -> c_int;    
 }