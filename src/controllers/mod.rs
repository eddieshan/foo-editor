pub mod edit_controller;

use crate::core::errors::EditorError;
use crate::text::keys::KeyBuffer;
use crate::models::editor::*;

type View = fn (&EditorState) -> Result<(), EditorError>;

type Controller = fn (KeyBuffer, usize, &mut EditorState) -> Result<(), EditorError>;

pub struct ActionResult {
    view: View,
    controller: Controller
}