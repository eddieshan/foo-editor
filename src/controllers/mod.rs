pub mod edit_controller;

use std::io::Write;
use crate::core::errors::EditorError;
use crate::text::keys::Key;
use crate::models::editor::*;

// It would better to constrain T as T: Write but unfortunately Rust 
// does not support generic bounds on type aliases.
pub type View<T> = fn (&mut T, &EditorState) -> Result<(), EditorError>;

pub struct ActionResult<T: Write> {
    pub view: View<T>,
    pub controller: fn (&Key, &mut EditorState) -> Result<ActionResult<T>, EditorError>
}