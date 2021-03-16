use crate::win::term::Position;

pub trait Cursor {
    fn up(&self) -> Position;

    fn down(&self) -> Position;

    fn right(&self) -> Position;

    fn left(&self) -> Position;

    fn crlf(&self) -> Position;

    fn is_valid(&self, limit: &Position) -> bool;
}

impl Cursor for Position {
    fn up(&self) -> Position {
        Position { x: self.x, y: self.y - 1 }
    }

    fn down(&self) -> Position {
        Position { x: self.x, y: self.y + 1 }
    }
    
    fn right(&self) -> Position {
        Position { x: self.x + 1, y: self.y }
    }
    
    fn left(&self) -> Position {
        Position { x: self.x - 1, y: self.y }
    }
    
    fn crlf(&self) -> Position {
        Position { x: 1, y: self.y + 1 }
    }

    fn is_valid(&self, limit: &Position) -> bool {
        self.x > 0 && self.y > 0 && self.x < limit.x && self.y < limit.y
    }
}