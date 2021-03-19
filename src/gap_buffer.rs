use std::io::Write;

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

    pub fn dump<T: Write>(&self, writer: &mut T)  {
        writer.write(&self.bytes[0..self.gap]);

        if self.end < BUFFER_LIMIT {
                writer.write(&self.bytes[self.end + 1..BUFFER_SIZE]);
        }
    }

    pub fn pos(&self) -> usize {
        self.gap
    }    
}