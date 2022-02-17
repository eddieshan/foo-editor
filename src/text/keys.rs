use std::io::{Read, Result};

pub const LF: u8 = 10;

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

pub struct Key {
    pub bytes: KeyBuffer,
    pub code: u32,
    pub length: usize
}

pub trait ReadKey {
    fn read_key(&mut self) -> Result<Key>;
}

impl<T: Read> ReadKey for T {
    fn read_key(&mut self) -> Result<Key> {
        let mut buffer: KeyBuffer = [0; 4];
        let length = self.read(&mut buffer)?;

        Ok(Key {
            bytes: buffer,
            code: u32::from_be_bytes(buffer),
            length: length
        })
    }
}