use std::io::Write;

use crate::core::errors::*;
use crate::term::vt100::*;
use crate::config::settings::*;
use crate::text::layout::*;
use crate::state::app::AppState;
use super::{plain_text, line_counter, status_bar};

pub fn render(buffer: &mut impl Write, settings: &Settings, state: &AppState) -> Result<(), EditorError> {
    let layout = Layout::from(&state.text_area.text(), &state.region, state.text_area.pos());

    plain_text::render(buffer, layout.text)?; 
    line_counter::render(buffer, layout.cursor.abs.y, layout.lines_range)?;
    status_bar::render(buffer, &layout.cursor.abs, &settings.window)?;

    buffer.pos(layout.cursor.rel.y, layout.cursor.rel.x + GUTTER_WIDTH)?;

    Ok(())
}