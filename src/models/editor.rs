use crate::term::common::*;
use crate::buffers::gap_buffer::GapBuffer;

pub struct EditorState {
    pub term_info: TermInfo,
    pub buffer: GapBuffer,
    pub text: Vec<u8>
}