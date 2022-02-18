use std::io::{Result, Write};
use crate::text::keys;
use crate::core::convert;

macro_rules! csi {
    ($($x:expr),*) => (
        [keys::ESC, b'[', $($x),*]
    );
    ($($x:expr,)*) => (esc![$($x),*])
}

pub const CLEAR: &[u8] = &csi!(b'2', b'J');
pub const RESET: &[u8] = &csi!(b'0', b'm');

pub const NEXT_LINE: &[u8] = &csi!(b'1', b'E');
pub const HOME: &[u8] = &csi!(b'H');

pub trait Vt100 {
    fn pos(&mut self, row: usize, col: usize) -> Result<()>;
}

impl<T: Write> Vt100 for T {
    fn pos(&mut self, row: usize, col: usize) -> Result<()> {
        let mut pos_seq = csi!(0, 0, 0, b';', 0, 0, 0, b'H');
        convert::to_slice_3(row, &mut pos_seq[2..5])?;
        convert::to_slice_3(col, &mut pos_seq[6..9])?;
        self.write(&pos_seq)?;
        Ok(())
    }
}