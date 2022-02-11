use crate::core::utils::*;
use crate::text::keys;

pub fn right(text: &[u8], pos: usize) -> usize {
    if pos < text.len() {
        pos + 1
    } else {
        pos
    }
}

pub fn left(text: &[u8], pos: usize) -> usize {
    if pos > 0 {
        pos - 1
    } else {
        pos
    }
}

pub fn down(text: &[u8], pos: usize) -> usize {
    if let Some(line_end) = text.aposition(keys::LF, pos) {
        let line_below_end = text.aposition(keys::LF, line_end + 1).unwrap_or(text.len());
        let line_above_end = text.raposition(keys::LF, pos).unwrap_or(0);
        let distance_to_start = pos - line_above_end;

        let offset = if line_above_end == 0 { 1 } else { 0 };
        let new_pos = line_end + distance_to_start + offset;
        
        std::cmp::min(new_pos, line_below_end)
    } else {
        pos
    }
}

pub fn up(text: &[u8], pos: usize) -> usize {
    if let Some(line_above_end) = text.raposition(keys::LF, pos) {
        let line_above_start = text.raposition(keys::LF, line_above_end - 1).unwrap_or(0);
        let distance_to_start = pos - line_above_end;

        let offset = if line_above_start == 0 { 1 } else { 0 };
        let new_pos = line_above_start + distance_to_start - offset;

        std::cmp::min(new_pos, line_above_end)
    } else {
        pos
    }
}

pub fn start(text: &[u8], pos: usize) -> usize {
    text.raposition(keys::LF, pos).map_or(0, |i| i + 1)
}

pub fn end(text: &[u8], pos: usize) -> usize {
    text.aposition(keys::LF, pos).unwrap_or(text.len())
}