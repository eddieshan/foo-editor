use crate::core::geometry::Size;
use crate::buffers::piece_chain::PieceChain;
use crate::core::utils::AbsPos;
use crate::text::keys::*;

struct Region {
    start: usize,
    end: usize
}

impl Region {

    fn slide_down(&mut self, text: &[u8], new_pos: usize, n_rows: usize) {
        self.end = text.apos(LF, new_pos).unwrap_or(text.len());
        self.start = text.rapos_n(LF, n_rows, self.end).unwrap_or(0);
    }
    
    fn slide_up(&mut self, text: &[u8], new_pos: usize, n_rows: usize) {
        self.start = text.rapos(LF, new_pos).unwrap_or(0);
        self.end = text.apos_n(LF, n_rows, self.start).unwrap_or(text.len());
    }
    
    pub fn slide(&mut self, text: &[u8], new_pos: usize, n_rows: usize) {
        if new_pos > self.end {
            self.slide_down(text, new_pos, n_rows)
        } else if new_pos < self.start {
            self.slide_up(text, new_pos, n_rows)
        } else {
            self.end = text.apos_n(LF, n_rows, self.start).unwrap_or(text.len());
        }
    }
}

pub struct EditorState {
    pub window_size: Size,
    buffer: PieceChain,
    pub text: Vec<u8>,
    clip_region: Region,
    pub pos: usize
}

impl EditorState {

    pub fn new(window_size: Size) -> Self {
        let screen_buffer_size = window_size.width*window_size.height;
        let buffer_size = screen_buffer_size*10;
        let n_pieces = buffer_size/2;

        EditorState {
            window_size: window_size,
            buffer: PieceChain::with_capacity(buffer_size, n_pieces),
            text: Vec::with_capacity(screen_buffer_size),
            clip_region: Region { start: 0, end: 0 },
            pos: 0
        }
    }

    fn refresh(&mut self) {
        self.text.clear();
        for chunk in &self.buffer {
            self.text.extend_from_slice(chunk);
        }
    }

    pub fn insert(&mut self, val: u8) {
        self.buffer.insert(val, self.pos);
        self.pos += 1;
        self.refresh();
        self.clip_region.slide(&self.text, self.pos, self.window_size.height);
    }

    pub fn erase(&mut self) {
        self.buffer.erase(self.pos);
        self.refresh();
    }

    pub fn go_erase(&mut self, mv: fn(&[u8], usize) -> usize) {
        self.go_to(mv);
        self.erase();
    }

    pub fn clipped_text(&self) -> &[u8] {
        &self.text[self.clip_region.start..self.clip_region.end]
    }

    pub fn go_to(&mut self, mv: fn(&[u8], usize) -> usize) {
        let new_pos = mv(&self.text, self.pos);
        self.clip_region.slide(&self.text, new_pos, self.window_size.height);
        self.pos = new_pos;
    }
}