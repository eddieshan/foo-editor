#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

impl Size {
    pub fn area(&self) -> usize {
        self.width*self.height
    }
}

pub struct Position {
    pub x: usize,
    pub y: usize
}