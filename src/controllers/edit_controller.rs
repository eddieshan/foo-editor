use std::io;
use crate::core::errors::*;

use crate::text::{keys, nav, keys::Key};
use crate::models::editor::EditorState;
use crate::views;
use super::ActionResult;

pub fn edit<T: io::Write>(key: &Key, state: &mut EditorState) -> Result<ActionResult<T>, EditorError> {

    match key.code {
        keys::CR        => state.insert(keys::LF),
        keys::UP        => state.go_to(nav::up),
        keys::DOWN      => state.go_to(nav::down),
        keys::RIGHT     => state.go_to(nav::right),
        keys::LEFT      => state.go_to(nav::left),
        keys::HTAB      => { },
        keys::LN_START  => state.go_to(nav::start),
        keys::LN_END    => state.go_to(nav::end),
        keys::DEL       => state.erase(),
        keys::BS        => state.go_erase(nav::left),
        _               => {
            if key.length == 1 {
                state.insert(key.bytes[0]);
            }
        }
    };

    Ok(ActionResult {
        view: views::edit::render,
        controller: super::edit_controller::edit
    })
}