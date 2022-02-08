use std::io::Write;

use crate::core::geometry::Position;
use crate::core::errors::*;
use crate::term::vt100::*;
use crate::models::editor::EditorState;
use super::gutter;
use super::status_bar;
use crate::config::settings;

pub fn render(buffer: &mut impl Write, state: &EditorState) -> Result<(), EditorError> {

    let (total_ln, lncol) = state.buffer.dump(buffer)?;

    gutter::render(buffer, lncol.y, total_ln)?;
    status_bar::render(buffer, &lncol, &state.term_info)?;
    
    let screen_pos = Position { x: lncol.x + settings::GUTTER_WIDTH, y: lncol.y };
    
    buffer.pos(screen_pos.y, screen_pos.x)?;

    Ok(())
}