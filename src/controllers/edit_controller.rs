use std::io;
use crate::core::errors::*;

use crate::text::{keys, nav, keys::Key};
use crate::models::editor::AppState;
use crate::views;
use super::ActionResult;

pub fn edit<T: io::Write>(key: &Key, state: &mut AppState) -> Result<ActionResult<T>, EditorError> {

    match key.code {
        keys::CR        => state.text_area.insert(keys::LF),
        keys::UP        => state.text_area.go_to(nav::up),
        keys::DOWN      => state.text_area.go_to(nav::down),
        keys::RIGHT     => state.text_area.go_to(nav::right),
        keys::LEFT      => state.text_area.go_to(nav::left),
        keys::HTAB      => { },
        keys::LN_START  => state.text_area.go_to(nav::start),
        keys::LN_END    => state.text_area.go_to(nav::end),
        keys::DEL       => state.text_area.erase(),
        keys::BS        => state.text_area.go_erase(nav::left),
        _               => {
            if key.length == 1 {
                state.text_area.insert(key.bytes[0]);
            }
        }
    };

    state.region.update(&state.text_area.text, state.text_area.pos);

    Ok(ActionResult {
        view: views::edit::render,
        controller: super::edit_controller::edit
    })
}