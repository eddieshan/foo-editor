pub const SEQ: &[u8] = b"\x1b[";

pub const CLEAR: &[u8] = b"\x1b[2J";
pub const RESET: &[u8] = b"\x1b[0m";

pub const COL: &[u8] = b"G";
pub const POS: &[u8] = b"H";
pub const NEXT_LINE: &[u8] = b"\x1b[1E";
pub const HOME: &[u8] = b"\x1b[H";
pub const SAVE_CURSOR: &[u8] = b"\x1b[s";
pub const RESTORE_CURSOR: &[u8] = b"\x1b[u";

pub const DEL_1: &[u8] = b"\x1b[1P";

pub const SET_FG: &[u8] = b"\x1b[38;2;";
pub const SET_BG: &[u8] = b"\x1b[48;2;";

pub const UNDERLINE: &[u8] = b"\x1b[4m";
pub const STRIKETROUGH: &[u8] = b"\x1b[9m";

pub const DEL: u32 = 0x1b5b337e;