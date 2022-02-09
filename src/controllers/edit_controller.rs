use std::io;
use crate::core::errors::*;

use crate::text::{keys, keys::Key};
use crate::models::editor::EditorState;
use crate::views;
use super::ActionResult;

pub fn edit<T: io::Write>(key: &Key, state: &mut EditorState) -> Result<ActionResult<T>, EditorError> {

    match key.code {
        keys::CR        => state.buffer.insert(keys::LINE_FEED),
        keys::UP        => state.buffer.up(),
        keys::DOWN      => state.buffer.down(),
        keys::RIGHT     => state.buffer.right(),
        keys::LEFT      => state.buffer.left(),
        keys::HTAB      => { },
        keys::LN_START  => state.buffer.ln_start(),
        keys::LN_END    => state.buffer.ln_end(),
        keys::DEL       => state.buffer.del_right(),
        keys::BS        => state.buffer.del_left(),
        _               => {
            if key.length == 1 {
                state.buffer.insert(key.bytes[0]);
            }
        }
    };

    Ok(ActionResult {
        view: views::edit::render,
        controller: super::edit_controller::edit
    })
}