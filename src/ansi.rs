use std::io::{Write};

pub const SEQ: &[u8] = b"\x1b[";

pub const CLEAR: &[u8] = b"\x1b[2J";
pub const RESET: &[u8] = b"\x1b[0m";

pub const POS: &[u8] = b"H";
pub const NEXT_LINE: &[u8] = b"\x1b[1E";
pub const HOME: &[u8] = b"\x1b[H";

pub const DEL_1: &[u8] = b"\x1b[1P";
pub const BACKDEL_1: &[u8] = b"\x1b[1D\x1b[1P";

pub const UNDERLINE: &[u8] = b"\x1b[4m";
pub const STRIKETROUGH: &[u8] = b"\x1b[9m";

pub const DEL: u32 = 0x1b5b337e;

pub fn pos(row: usize, col: usize, buffer: &mut impl Write) {
    buffer.write(SEQ);
    print!("{};{}", row, col);
    buffer.write(POS);
}