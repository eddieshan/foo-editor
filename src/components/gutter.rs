use std::io::{Result, Write};

use crate::term::vt100;
use crate::core::convert;
use crate::config::theme;

pub fn render(buffer: &mut impl Write, ln: usize, total_ln: usize) -> Result<()> {
    buffer.write(vt100::HOME)?;
    buffer.write(theme::GUTTER_DEFAULT)?;

    let mut line_number = [b' '; 3];

    for i in 1..=total_ln {
        convert::to_slice_3(i, &mut line_number)?;
        if i == ln {
            buffer.write(theme::GUTTER_HIGHLIGHT)?;
            buffer.write(&line_number)?;
            buffer.write(theme::GUTTER_DEFAULT)?;
        } else {
            buffer.write(&line_number)?;
        }            
        
        buffer.write(vt100::NEXT_LINE)?;
    }

    buffer.write(vt100::RESET)?;

    Ok(())
}
