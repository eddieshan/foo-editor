use std::io::Write;

use crate::core::{errors::*, geometry::Position};
use crate::term::vt100::*;
use crate::config::settings;
use crate::models::editor::EditorState;
use super::{gutter, status_bar};

pub fn render(buffer: &mut impl Write, state: &EditorState) -> Result<(), EditorError> {

    let (total_ln, lncol) = state.buffer.dump(buffer)?;

    gutter::render(buffer, lncol.y, total_ln)?;
    status_bar::render(buffer, &lncol, &state.term_info)?;
    
    let screen_pos = Position { x: lncol.x + settings::GUTTER_WIDTH, y: lncol.y };
    
    buffer.pos(screen_pos.y, screen_pos.x)?;

    Ok(())
}