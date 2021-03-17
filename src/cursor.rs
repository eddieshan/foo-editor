use crate::core::{Size, Position};
use crate::theme;

pub struct Cursor {
    pos: usize,
    limit: usize,
    width: usize
}

impl Cursor {

    pub fn new(screen_size: &Size) -> Cursor {
        let width = screen_size.width - theme::GUTTER_WIDTH;
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
            x: (self.pos % self.width) + 1 + theme::GUTTER_WIDTH, 
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
            self.pos += self.width;
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
}