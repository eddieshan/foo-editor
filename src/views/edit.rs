use std::io::Write;

use crate::core::errors::*;
use crate::term::vt100::*;
use crate::config::settings;
use crate::models::editor::EditorState;
use super::{plain_text, line_counter, status_bar};

pub fn render(buffer: &mut impl Write, state: &EditorState) -> Result<(), EditorError> {
    let layout = state.region.layout(&state.text, state.window.height - 1);

    plain_text::render(buffer, layout.text)?; 
    line_counter::render(buffer, layout.cursor.abs.y, layout.lines_range)?;
    status_bar::render(buffer, &layout.cursor.abs, &state.window)?;

    buffer.pos(layout.cursor.rel.y, layout.cursor.rel.x + settings::GUTTER_WIDTH)?;

    Ok(())
}