use std::io::Write;

use crate::{settings, keys};
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

    fn ln_start(&mut self) -> usize {
        let mut pos = self.gap;
        while pos > 0 {
            pos -= 1;

            if self.bytes[pos] == keys::LINE_FEED {
                return pos + 1;
            }
        }

        0
    }

    fn next_new_line(&mut self) -> Option<usize> {
        let mut pos = self.end + 1;
        while pos < BUFFER_SIZE {
            if self.bytes[pos] == keys::LINE_FEED {
                return Some(pos)
            }
            pos += 1;
        }
        None
    }

    pub fn up(&mut self) {
        let mut pos = self.gap;
        let mut ln_count = 0;
        let mut col_offset = [0_usize; 2];

        while pos > 0 && ln_count < 2 {

            pos -= 1;

            if self.bytes[pos] == keys::LINE_FEED {
                ln_count += 1;
            } else {
                col_offset[ln_count] += 1;
            }
        }

        if ln_count > 0 {
            let new_gap = match pos {
                0 => pos + col_offset[0],
                _ => pos + col_offset[0] + 1
            };
            let size = self.gap - new_gap;
            let new_end = self.end - size;
            self.bytes.copy_within(new_gap..self.gap, new_end + 1);
            self.gap = new_gap;
            self.end = new_end;
        }
    }

    pub fn down(&mut self) {
        if let Some(pos) = self.next_new_line() {
            let col = self.gap - self.ln_start();

            let new_end = match pos + col {
                v if v > BUFFER_LIMIT => BUFFER_LIMIT,
                v                     => v
            };

            let size = new_end - self.end;
            let new_gap = self.gap + size;

            self.bytes.copy_within(self.end + 1..=new_end, self.gap);            

            self.gap = new_gap;
            self.end = new_end;
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
            if self.bytes[i] == keys::LINE_FEED {
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