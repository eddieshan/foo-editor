use crate::core::{Size, Position};
use crate::settings;

pub struct Cursor {
    pos: usize,
    limit: usize,
    width: usize
}

impl Cursor {

    pub fn new(screen_size: &Size) -> Cursor {
        let width = screen_size.width - settings::GUTTER_WIDTH;
        let height = screen_size.height - 1;

        Cursor { pos: 0, width: width, limit: width*height - 1 }
    }

    pub fn pos(&self) -> Position {
        Position { 
            x: (self.pos % self.width) + 1, 
            y: (self.pos / self.width) + 1
        }
    }

    pub fn screen_pos(&self) -> Position {
        Position { 
            x: (self.pos % self.width) + 1 + settings::GUTTER_WIDTH, 
            y: (self.pos / self.width) + 1
        }
    }

    pub fn up(&mut self) -> bool {
        if self.pos >= self.width {
            self.pos -= self.width;
            return true;
        }
        return false;
    }

    pub fn down(&mut self) -> bool {
        let next_pos = self.pos + self.width;
        if next_pos < self.limit {
            self.pos = next_pos;
            return true;
        }
        return false;
    }
    
    pub fn right(&mut self) -> bool {
        if self.pos < self.limit {
            self.pos += 1;
            return true;
        }
        return false;
    }
    
    pub fn left(&mut self) -> bool {
        if self.pos > 0 {
            self.pos -= 1;
            return true;
        }
        return false;
    }
    
    pub fn crlf(&mut self) -> bool {
        let new_pos = self.pos + self.width - (self.pos % self.width);
        if new_pos < self.limit {
            self.pos = new_pos;
            return true;
        }
        return false;
    }

    pub fn htab(&mut self) -> bool {
        let next_pos = self.pos + settings::HORIZONTAL_TAB;
        if self.pos < self.limit {
            self.pos = next_pos;
            return true;
        }
        return false;
    }

    pub fn ln_start(&mut self) -> bool {
        self.pos = self.pos - (self.pos % self.width);
        true
    }

    pub fn ln_end(&mut self) -> bool {
        self.pos = self.pos + self.width - (self.pos % self.width) - 1;
        true
    }
}