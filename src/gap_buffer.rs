use std::io::Write;

use crate::settings;
use crate::core::Position;

const KB: usize = 1024;
const BLOCK_SIZE: usize = 10*KB;
const BUFFER_SIZE: usize = 3*BLOCK_SIZE;
const BUFFER_LIMIT: usize = BUFFER_SIZE - 1;

pub struct GapBuffer {
    bytes: [u8; BUFFER_SIZE],
    gap: usize,
    end: usize
}

impl GapBuffer {
    pub fn new() -> GapBuffer {
        GapBuffer {
            bytes: [0; BUFFER_SIZE],
            gap: 0,
            end: BUFFER_LIMIT
        }
    }

    pub fn insert(&mut self, byte: u8) {
        self.bytes[self.gap] = byte;
        self.gap += 1;
    }

    pub fn insert_n(&mut self, bytes: &[u8]) {
        let n = bytes.len();
        let next_gap = self.gap + n;
        self.bytes[self.gap..next_gap].copy_from_slice(bytes);
        self.gap = next_gap;
    }

    pub fn right(&mut self) {
        if self.end < BUFFER_LIMIT {
            self.end += 1;
            self.bytes[self.gap] = self.bytes[self.end];
            self.gap += 1;            
        }
    }

    pub fn left(&mut self) {
        if self.gap > 0 {
            self.gap -= 1;
            self.bytes[self.end] = self.bytes[self.gap];            
            self.end -= 1;
        }
    }

    pub fn del_right(&mut self) {
        if self.end < BUFFER_LIMIT {
            self.end += 1;
        }
    }

    pub fn del_left(&mut self) {
        if self.gap > 0 {
            self.gap -= 1;
        }
    }    

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[0..self.gap]
    }

    fn dump_lines<T: Write>(&self, writer: &mut T, from: usize, to: usize) -> (usize, usize)  {
        let mut ln_start = from;
        let mut ln_count = 0;
        for i in from..to {
            if self.bytes[i] == 10 {
                writer.write(&self.bytes[ln_start..i]);
                writer.write(settings::LINE_FEED);
                ln_count += 1;
                ln_start = i + 1;
            }
        }

        writer.write(&self.bytes[ln_start..to]);

        (ln_count, to - ln_start)
    }

    pub fn dump<T: Write>(&self, writer: &mut T) -> (usize, Position)  {
        let (gap_ln, gap_col) = self.dump_lines(writer, 0, self.gap);
        let lncol = Position { x: gap_col + 1, y: gap_ln + 1 };

        let total_ln = match self.end < BUFFER_LIMIT {
            true => {
                let (end_ln, _) = self.dump_lines(writer, self.end + 1, BUFFER_SIZE);
                gap_ln + end_ln + 1
            },
            false => gap_ln + 1
        };

        (total_ln, lncol)
    }
}