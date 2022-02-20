use crate::core::collections::Search;
use crate::text::keys::*;

pub struct Region {
    pub start: usize,
    pub page_size: usize
}

impl Region {
    pub fn update(&mut self, text: &[u8], new_pos: usize) {
        if new_pos > self.start && (&text[self.start..new_pos]).at_least(LF, self.page_size + 1) {
            self.start = text.rpos_n(LF, self.page_size - 1, new_pos).map_or(0, |v| v + 1);
        } else if new_pos < self.start {
            self.start = text.rpos(LF, new_pos).unwrap_or(0);
        }
    }

    // fn clip<'a>(&self, text: &'a [u8]) -> &'a [u8] {
    //     let end = text.pos_n(LF, self.page_size, self.start).unwrap_or(text.len());
    //     &text[self.start..end]
    // }

    // fn before<'a>(&self, text: &'a [u8]) -> &'a [u8] {
    //     &text[..self.start]
    // }

    // fn abs<'a>(&self, text: &'a [u8]) -> &'a [u8] {
    //     &text[..self.pos]
    // }

    // fn rel<'a>(&self, text: &'a [u8]) -> &'a [u8] {
    //     &text[self.start..self.pos]
    // }    

    // pub fn layout<'a>(&self, text:&'a [u8]) -> TextLayout<'a> {
    //     let clipped_text = self.clip(text);
    //     let start_line = self.before(text).count(LF) + 1;
    //     let end_line = start_line + clipped_text.count(LF) + 1;

    //     // TODO: calling region.abs and region.rel is inefficient since they will
    //     // pass twice over the range region.start..region.pos. This needs to be 
    //     // replaced by an incremental calculation.
    //     TextLayout {
    //         text: clipped_text,
    //         cursor: Cursor {
    //             abs: self.abs(text).last_pos(),
    //             rel: self.rel(text).last_pos()
    //         },
    //         lines_range: start_line..end_line
    //     }
    // }
}