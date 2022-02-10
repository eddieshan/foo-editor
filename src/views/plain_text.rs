use std::io::Write;

use crate::core::errors::*;
use crate::term::vt100::*;
use crate::text::keys;
use crate::config::settings;

pub fn render(buffer: &mut impl Write, text: &[u8]) -> Result<(), EditorError> {
    
    let col = settings::GUTTER_WIDTH + 1;
    buffer.pos(0, col)?;

    let mut last_cr = 0;
    let last = text.len() - 1;

    for i in 0..text.len() {
        let is_line_break = text[i] == keys::LINE_FEED;

        if is_line_break || i == last {
            let end_of_line = match is_line_break {
                true  => i,
                false => i + 1
            };
            buffer.write(&text[last_cr..end_of_line])?;

            if is_line_break {
                buffer.write(settings::LINE_FEED)?;
                last_cr = i + 1;
            }
        }
    }

    Ok(())
}