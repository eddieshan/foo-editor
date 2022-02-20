use std::io::{Result, Write};

use crate::core::{convert, geometry::*};
use crate::term::{vt100, vt100::Vt100};

pub fn render(buffer: &mut impl Write, cursor: &Position, window: &Size) -> Result<()> {
    let mut caption: [u8; 7] = [0, 0, 0, b':', 0, 0, 0];
    convert::to_slice_3(cursor.x, &mut caption[0..3])?;
    convert::to_slice_3(cursor.y, &mut caption[4..7])?;

    let start_col = window.width + 1 - caption.len();

    buffer.pos(window.height, start_col)?;
    buffer.write(&caption)?;
    buffer.write(vt100::RESET)?;
    
    Ok(())
}