use std::ops::Range;
use crate::core::collections::Search;
use crate::core::geometry::*;
use crate::text::{keys::*, nav::*};
use super::region::*;

pub struct Cursor {
    pub abs: Position,
    pub rel: Position
}

pub struct TextLayout<'a> {
    pub text: &'a [u8],
    pub cursor: Cursor,
    pub lines_range: Range<usize>
}

pub trait Text {
    fn clip<'a>(&'a self, region: &Region) -> &'a [u8];
    fn layout<'a>(&'a self, region: &Region, pos: usize) -> TextLayout<'a>;    
}

impl Text for [u8] {
    fn clip<'a>(&'a self, region: &Region) -> &'a [u8] {
        let end = self.pos_n(LF, region.page_size, region.start).unwrap_or(self.len());
        &self[region.start..end]
    }

    fn layout<'a>(&'a self, region: &Region, pos: usize) -> TextLayout<'a> {
        let clipped_text = self.clip(region);
        let start_line = self[..region.start].count(LF) + 1;
        let end_line = start_line + clipped_text.count(LF) + 1;

        // TODO: calling region.abs and region.rel is inefficient since they will
        // pass twice over the range region.start..region.pos. This needs to be 
        // replaced by an incremental calculation.
        TextLayout {
            text: clipped_text,
            cursor: Cursor {
                abs: self[..pos].last_pos(),
                rel: self[region.start..pos].last_pos()
            },
            lines_range: start_line..end_line
        }
    }    
}