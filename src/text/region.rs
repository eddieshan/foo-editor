use crate::core::collections::Search;
use crate::text::keys::*;

pub struct Region {
    pub start: usize,
    pub page_size: usize
}

impl Region {
    pub fn update(&mut self, text: &[u8], new_pos: usize) {
        if new_pos > self.start && text[self.start..new_pos].at_least(LF, self.page_size) {
            self.start = text.rpos_n(LF, self.page_size, new_pos).map_or(0, |v| v + 1);
        } else if new_pos < self.start {
            self.start = text.rpos(LF, new_pos).unwrap_or(0);
        }
    }
}