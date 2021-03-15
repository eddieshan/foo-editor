use std::io::{Stdout, Result, Write};

use crate::{ansi, keys, theme};
use crate::win::term::TermInfo;

pub fn render(stdout: &mut Stdout, info: &TermInfo) -> Result<()> {
    stdout.write(ansi::HOME)?;
    stdout.write(theme::GUTTER_DEFAULT)?;

    let height = info.screen_size.height;
    let cursor_y = info.cursor.y + 1;

    for i in 1..height {
        if i == cursor_y {
            stdout.write(theme::GUTTER_HIGHLIGHT)?;
            print!("{:>3} ", i);
            stdout.write(theme::GUTTER_DEFAULT)?;
        } else {
            print!("{:>3} ", i);
        }            
        
        stdout.write(ansi::NEXT_LINE)?;
    }

    stdout.write(ansi::RESET)?;

    Ok(())
}
