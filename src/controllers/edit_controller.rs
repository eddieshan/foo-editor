use std::io;
use crate::core::errors::*;

use crate::text::keys::KeyBuffer;
use crate::text::keys;
use crate::models::editor::EditorState;
use crate::views;
use super::ActionResult;

pub fn edit<T: io::Write>(key: KeyBuffer, length: usize, state: &mut EditorState) -> Result<ActionResult<T>, EditorError> {

    let code = u32::from_be_bytes(key); // Conversion has to be big endian to match the input sequence.

    match code {
        keys::CTRL_Q    => { },
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
            if length == 1 {
                state.buffer.insert(key[0]);
            }
        }
    };

    Ok(ActionResult {
        view: views::edit::render,
        controller: super::edit_controller::edit
    })
}