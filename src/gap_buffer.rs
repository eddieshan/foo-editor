use std::cmp;
use std::ops::Range;
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

    fn find_lf<T: Iterator<Item = usize>>(&self, span: T, lf_pos:  &mut [usize; 2]) -> usize {
        let mut lf_count = 0;

        for pos in span {
            lf_pos[lf_count] = pos;
            if self.bytes[pos] == keys::LINE_FEED {
                lf_count += 1;
                if lf_count == 2 {
                    return lf_count;
                }
            }
        }

        lf_count
    }

    pub fn up(&mut self) {
        let mut lf_pos = [0_usize; 2];  
        let lf_count = self.find_lf((0..self.gap).rev(), &mut lf_pos);

        if lf_count > 0 {
            let col = match lf_count {
                1 => self.gap - lf_pos[0] - 1,
                _ => self.gap - lf_pos[0]
            };
            let col_above = lf_pos[0] - lf_pos[1];

            let new_col = cmp::min(col, col_above);
            let new_gap = lf_pos[1] + new_col;
            let size = self.gap - new_gap;
            let new_end = self.end - size;

            self.bytes.copy_within(new_gap..self.gap, new_end + 1);
            self.gap = new_gap;
            self.end = new_end;
        }
    }

    pub fn down(&mut self) {
        let mut lf_pos = [0_usize; 2];
        let lf_count = self.find_lf(self.end + 1..BUFFER_SIZE, &mut lf_pos);

        if lf_count > 0 {
            let col = self.gap - self.ln_start();
            let col_below = lf_pos[1] - lf_pos[0];

            let new_col = cmp::min(col, col_below);
            let new_end = lf_pos[0] + new_col;
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