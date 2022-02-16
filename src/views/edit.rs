use std::io::Write;

use crate::core::{errors::*, geometry::Position};
use crate::term::vt100::*;
use crate::text::navigation;
use crate::config::settings;
use crate::models::editor::EditorState;
use super::{plain_text, gutter, status_bar};

pub fn render(buffer: &mut impl Write, state: &EditorState) -> Result<(), EditorError> {
    let absolute_pos = navigation::cursor(&state.text[..state.region.pos]);
    let (lines_range, text) = state.region.clip(&state.text, state.window_size.height - 1);

    if text.len() > 0 {
        plain_text::render(buffer, text)?;
    }

    gutter::render(buffer, absolute_pos.y, lines_range)?;
    status_bar::render(buffer, &absolute_pos, &state.window_size)?;

    let relative_pos = navigation::cursor(&state.text[state.region.start..state.region.pos]);
    let screen_pos = Position { x: relative_pos.x + settings::GUTTER_WIDTH, y: relative_pos.y };
    
    buffer.pos(screen_pos.y, screen_pos.x)?;

    Ok(())
}