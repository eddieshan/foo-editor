use std::io::{Result, Write};

use crate::core::*;
use crate::{ansi, keys, theme};

const MAX_LINE_WIDTH: usize = 200;
const WHITESPACE_LINE: [u8; MAX_LINE_WIDTH] = [keys::WHITESPACE; MAX_LINE_WIDTH];

pub fn render(buffer: &mut impl Write, cursor: &Position, info: &TermInfo) -> Result<()> {
    buffer.write(theme::STATUS_DEFAULT)?;

    let status = format!("{}:{},{}:{} ", cursor.x, cursor.y, info.buffer_size.width, info.buffer_size.height);

    let last_col = info.screen_size.width + 1;
    let start_col = last_col - status.len();

    ansi::pos(info.screen_size.height, 0, buffer);
    buffer.write(&WHITESPACE_LINE[0..info.screen_size.width])?;
    ansi::pos(info.screen_size.height, start_col, buffer);
    print!("{}", status);

    buffer.write(ansi::RESET)?;
    
    Ok(())
}