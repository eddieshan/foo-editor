use std::io::Write;

use crate::core::{errors::*, geometry::Position};
use crate::term::vt100::*;
use crate::text::keys;
use crate::config::settings;
use crate::models::editor::EditorState;
use super::{plain_text, gutter, status_bar};

pub fn render(buffer: &mut impl Write, state: &EditorState) -> Result<(), EditorError> {
    let n_lines = state.text.iter()
        .filter(|&&v| v == keys::LF)
        .count() + 1;

    let mut cursor = Position { x: 1, y: 1 };

    state.text[0..state.pos].iter().for_each(|&v| {
        if v == keys::LF {
            cursor.y += 1;
            cursor.x = 1;
        } else {
            cursor.x += 1;
        }
    });

    if state.text.len() > 0 {
        plain_text::render(buffer, &state.text)?;
    }

    gutter::render(buffer, cursor.y, n_lines)?;
    status_bar::render(buffer, &cursor, &state.term_info)?;
    
    let screen_pos = Position { x: cursor.x + settings::GUTTER_WIDTH, y: cursor.y };
    
    buffer.pos(screen_pos.y, screen_pos.x)?;

    Ok(())
}
