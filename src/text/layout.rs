use std::ops::Range;
use crate::core::collections::Search;
use crate::core::geometry::*;
use crate::text::{keys::*, nav::*};
use super::region::*;

pub struct Cursor {
    pub abs: Position,
    pub rel: Position
}

pub struct Layout<'a> {
    pub text: &'a [u8],
    pub cursor: Cursor,
    pub lines_range: Range<usize>
}

fn clip<'a>(text: &'a [u8], region: &Region) -> &'a [u8] {
    let end = text.pos_n(LF, region.page_size, region.start).unwrap_or(text.len());
    &text[region.start..end]
}

impl<'a> Layout<'a> {
    pub fn from(text: &'a [u8], region: &Region, pos: usize) -> Self {
        let clipped_text = clip(text, region);
        let start_line = text[..region.start].count(LF) + 1;
        let end_line = start_line + clipped_text.count(LF) + 1;

        // TODO: this calculation of abs and rel position is inefficient since 
        // it will incur on two passes over the range region.start..region.pos. 
        // This needs to be replaced by an incremental calculation.
        Layout {
            text: clipped_text,
            cursor: Cursor {
                abs: text[..pos].last_pos(),
                rel: text[region.start..pos].last_pos()
            },
            lines_range: start_line..end_line
        }
    }
}