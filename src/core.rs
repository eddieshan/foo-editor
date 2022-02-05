#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

pub struct Position {
    pub x: usize,
    pub y: usize
}

pub struct TermInfo {
    pub buffer_size: Size,
    pub screen_size: Size
}