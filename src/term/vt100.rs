use std::io::{Result, Write};
use crate::core::convert;

pub const SEQ: &[u8] = b"\x1b[";

pub const CLEAR: &[u8] = b"\x1b[2J";
pub const RESET: &[u8] = b"\x1b[0m";

pub const POS: &[u8] = b"H";
pub const NEXT_LINE: &[u8] = b"\x1b[1E";
pub const HOME: &[u8] = b"\x1b[H";

pub fn pos(row: usize, col: usize, buffer: &mut impl Write) -> Result<()> {
    buffer.write(SEQ)?;
    let mut pos_seq: [u8; 7] = [0, 0, 0, b';', 0, 0, 0];
    convert::to_slice_3(row, &mut pos_seq[0..3])?;
    convert::to_slice_3(col, &mut pos_seq[4..7])?;
    buffer.write(&pos_seq)?;
    buffer.write(POS)?;
    Ok(())
}