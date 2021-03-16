use std::io::{Stdout, Result, Write};

use crate::{ansi, theme};
use crate::core::*;

pub fn render(stdout: &mut Stdout, cursor: &Position, info: &TermInfo) -> Result<()> {
    stdout.write(ansi::HOME)?;
    stdout.write(theme::GUTTER_DEFAULT)?;

    for i in 1..info.screen_size.height {
        if i == cursor.y {
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
