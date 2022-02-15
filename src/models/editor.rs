use std::ops::Range;
use crate::core::utils::AbsPos;
use crate::text::keys::*;
use crate::term::common::*;
use crate::buffers::piece_chain::PieceChain;

pub struct EditorState {
    pub term_info: TermInfo,
    buffer: PieceChain,
    pub text: Vec<u8>,
    pub clip_region: Range<usize>,
    pub pos: usize
}

fn slide_window_down(text: &[u8], new_pos: usize, n_rows: usize) -> Range<usize> {
    let new_end = text.apos(LF, new_pos).unwrap_or(text.len());
    let new_start = text.rapos_n(LF, n_rows, new_end).unwrap_or(0);
    Range { start: new_start, end: new_end }
}

fn slide_window_up(text: &[u8], new_pos: usize, n_rows: usize) -> Range<usize> {
    let new_start = text.rapos(LF, new_pos).unwrap_or(0);
    let new_end = text.apos_n(LF, n_rows, new_start).unwrap_or(text.len());
    Range { start: new_start, end: new_end }
}

impl EditorState {

    pub fn new(term_info: TermInfo) -> Self {
        let screen_buffer_size = term_info.screen_size.width*term_info.screen_size.height;
        let buffer_size = screen_buffer_size*10;
        let n_pieces = buffer_size/2;

        EditorState {
            term_info: term_info,
            buffer: PieceChain::with_capacity(buffer_size, n_pieces),
            text: Vec::with_capacity(screen_buffer_size),
            clip_region: Range { start: 0, end: 0 },
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
        let new_pos = mv(&self.text, self.pos);

        if new_pos > self.clip_region.end {
            self.clip_region = slide_window_down(&self.text, new_pos, self.term_info.screen_size.height);
        } else if new_pos < self.clip_region.start {
            self.clip_region = slide_window_up(&self.text, new_pos, self.term_info.screen_size.height);
        }
        self.pos = new_pos;
    }

    // fn update_pos(&mut self, new_pos: usize) {
    //     if new_pos > self.clip_region.end {
    //         self.clip_region = slide_window_down(&self.text, new_pos, self.term_info.screen_size.height);
    //     } else if new_pos < self.clip_region.start {
    //         self.clip_region = slide_window_up(&self.text, new_pos, self.term_info.screen_size.height);
    //     }
    
    //     self.pos = new_pos;
    // }
}
