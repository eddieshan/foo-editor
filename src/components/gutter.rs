use std::io::{Result, Write};

use crate::{ansi, theme};

pub fn render(buffer: &mut impl Write, ln: usize, total_ln: usize) -> Result<()> {
    buffer.write(ansi::HOME)?;
    buffer.write(theme::GUTTER_DEFAULT)?;

    for i in 1..=total_ln {
        if i == ln {
            buffer.write(theme::GUTTER_HIGHLIGHT)?;
            print!("{:>3} ", i);
            buffer.write(theme::GUTTER_DEFAULT)?;
        } else {
            print!("{:>3} ", i);
        }            
        
        buffer.write(ansi::NEXT_LINE)?;
    }

    buffer.write(ansi::RESET)?;

    Ok(())
}
