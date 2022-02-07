use std::io::{Result, Write};

use crate::core::geometry::Position;
use crate::core::convert;
use crate::text::keys;
use crate::term::*;
use crate::term::vt100::*;
use crate::config::theme;

const MAX_LINE_WIDTH: usize = 200;
const WHITESPACE_LINE: [u8; MAX_LINE_WIDTH] = [keys::WHITESPACE; MAX_LINE_WIDTH];

pub fn render(buffer: &mut impl Write, cursor: &Position, info: &TermInfo) -> Result<()> {
    buffer.write(theme::STATUS_DEFAULT)?;

    let mut caption: [u8; 7] = [0, 0, 0, b':', 0, 0, 0];
    convert::to_slice_3(cursor.x, &mut caption[0..3])?;
    convert::to_slice_3(cursor.y, &mut caption[4..7])?;

    let last_col = info.screen_size.width + 1;
    let start_col = last_col - caption.len();

    buffer.pos(info.screen_size.height, 0)?;
    buffer.write(&WHITESPACE_LINE[0..info.screen_size.width])?;
    buffer.pos(info.screen_size.height, start_col)?;

    buffer.write(&caption)?;

    buffer.write(vt100::RESET)?;
    
    Ok(())
}