pub mod edit_controller;

use std::io::Write;
use crate::core::errors::EditorError;
use crate::text::keys::KeyBuffer;
use crate::models::editor::*;

// It would better to use type aliases for view and controller
// but unfortunately Rust does not support generic bounds on type
// aliases so T: Write cannot be declared.
pub struct ActionResult<T: Write> {
    view: fn (&mut T, &EditorState) -> Result<(), EditorError>,
    controller: fn (KeyBuffer, usize, &mut EditorState) -> Result<ActionResult<T>, EditorError>
}