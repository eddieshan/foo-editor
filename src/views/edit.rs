use std::io::Write;

use crate::core::{errors::*, geometry::Position};
use crate::term::vt100::*;
use crate::text::navigation;
use crate::config::settings;
use crate::models::editor::EditorState;
use super::{plain_text, gutter, status_bar};

pub fn render(buffer: &mut impl Write, state: &EditorState) -> Result<(), EditorError> {
    let n_lines = navigation::n_lines(&state.text);
    let cursor = navigation::cursor(&state.text[0..state.pos]);

    let text = state.clipped_text();
    if text.len() > 0 {
        plain_text::render(buffer, text)?;
    }

    gutter::render(buffer, cursor.y, n_lines)?;
    status_bar::render(buffer, &cursor, &state.window_size)?;
    
    let screen_pos = Position { x: cursor.x + settings::GUTTER_WIDTH, y: cursor.y };
    
    buffer.pos(screen_pos.y, screen_pos.x)?;

    Ok(())
}
