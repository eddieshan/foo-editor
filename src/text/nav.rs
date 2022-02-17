use crate::core::utils::*;
use crate::core::geometry::*;
use crate::text::keys::*;

pub fn cursor(text: &[u8]) -> Position {
    let mut cursor = Position { x: 1, y: 1 };

    for &v in text {
        if v == LF {
            cursor.y += 1;
            cursor.x = 1;
        } else {
            cursor.x += 1;
        }
    };

    cursor
}

pub fn n_lines(text: &[u8]) -> usize {
    text.iter().filter(|&&v| v == LF).count() + 1
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