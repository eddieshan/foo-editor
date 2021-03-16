use std::convert::{TryFrom};
use std::io;
use std::io::{Stdout, Result, Read, Write};

use crate::{ansi, keys, theme};
use crate::win::term::{Position, TermInfo};

const MAX_LINE_WIDTH: usize = 200;
const WHITESPACE_LINE: [u8; MAX_LINE_WIDTH] = [keys::WHITESPACE; MAX_LINE_WIDTH];

pub fn render(stdout: &mut Stdout, cursor: &Position, info: &TermInfo) -> Result<()> {
    stdout.write(theme::STATUS_DEFAULT)?;

    let status = format!("{}:{},{}:{} ", cursor.x, cursor.y, info.buffer_size.width, info.buffer_size.height);

    let last_col = info.screen_size.width + 1;
    let start_col = last_col - status.len();

    ansi::pos(info.screen_size.height, 0, stdout);
    stdout.write(&WHITESPACE_LINE[0..info.screen_size.width])?;
    ansi::pos(info.screen_size.height, start_col, stdout);
    print!("{}", status);

    stdout.write(ansi::RESET)?;
    
    Ok(())
}