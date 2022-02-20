use crate::core::geometry::Size;

pub const GUTTER_WIDTH: usize = 5;

pub const LF: &[u8] = b"\x1b[1E\x1b[6G";

pub struct Settings {
    pub window: Size
}