use crate::core::geometry::*;
use crate::buffers::piece_chain::PieceChain;

pub struct TextArea {
    buffer: PieceChain,
    pub pos: usize,
    pub text: Vec<u8>
}

impl TextArea {

    pub fn new(window: Size) -> Self {
        let screen_buffer_size = window.area();
        let buffer_size = screen_buffer_size*10;
        let n_pieces = buffer_size/2;

        TextArea {
            buffer: PieceChain::with_capacity(buffer_size, n_pieces),
            text: Vec::with_capacity(screen_buffer_size),
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
        self.refresh();
        self.pos += 1;
    }

    pub fn erase(&mut self) {
        self.buffer.erase(self.pos);
        self.refresh();
    }

    pub fn go_erase(&mut self, mv: fn(&[u8], usize) -> usize) {
        self.go_to(mv);
        self.erase();
    }

    pub fn go_to(&mut self, mv: fn(&[u8], usize) -> usize) {
        self.pos = mv(&self.text, self.pos);
    }
}