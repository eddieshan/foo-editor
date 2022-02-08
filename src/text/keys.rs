pub const WHITESPACE: u8 = 32;
pub const LINE_FEED: u8 = 10;

pub const CTRL_Q: u32   = 0x11000000;
pub const CR: u32       = 0x0d000000;
pub const BS: u32       = 0x7f000000;

pub const UP: u32       = 0x1b5b4100;
pub const DOWN: u32     = 0x1b5b4200;
pub const RIGHT: u32    = 0x1b5b4300;
pub const LEFT: u32     = 0x1b5b4400;
pub const HTAB: u32     = 0x09000000;

pub const LN_START: u32 = 0x1b5b4800;
pub const LN_END: u32   = 0x1b5b4600;

pub const DEL: u32      = 0x1b5b337e;

pub type KeyBuffer = [u8; 4];
