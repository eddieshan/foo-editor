use crate::core::collections::*;
use crate::core::geometry::*;
use crate::text::keys::*;

pub trait Nav {
    fn last_pos(&self) -> Position;
}

impl Nav for [u8] {
    fn last_pos(&self) -> Position {
        let mut last_pos = Position { x: 1, y: 1 };

        for &v in self {
            if v == LF {
                last_pos.y += 1;
                last_pos.x = 1;
            } else {
                last_pos.x += 1;
            }
        }
    
        last_pos
    }
}

pub fn right(text: &[u8], pos: usize) -> usize {
    if pos < text.len() {
        pos + 1
    } else {
        pos
    }
}

pub fn left(_: &[u8], pos: usize) -> usize {
    if pos > 0 {
        pos - 1
    } else {
        pos
    }
}

pub fn down(text: &[u8], pos: usize) -> usize {
    if let Some(eol) = text.apos(LF, pos) {
        let eol_below = text.apos(LF, eol + 1).unwrap_or(text.len());
        let offset = text.rapos(LF, pos).map_or(pos + 1, |eol_above| pos - eol_above);
        let new_pos = eol + offset;
        
        std::cmp::min(new_pos, eol_below)
    } else {
        pos
    }
}

pub fn up(text: &[u8], pos: usize) -> usize {
    if let Some(eol_above) = text.rapos(LF, pos) {
        let offset = pos - eol_above;
        let new_pos = text.rapos(LF, eol_above).map_or(offset - 1, |sol_above| offset + sol_above);

        std::cmp::min(new_pos, eol_above)
    } else {
        pos
    }
}

pub fn start(text: &[u8], pos: usize) -> usize {
    text.rapos(LF, pos).map_or(0, |i| i + 1)
}

pub fn end(text: &[u8], pos: usize) -> usize {
    text.apos(LF, pos).unwrap_or(text.len())
}