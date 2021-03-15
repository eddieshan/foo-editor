pub const GUTTER_WIDTH: usize = 5;

// pub const GUTTER_DEFAULT: &[u8] = b"\x1b[38;2;100;100;100m\x1b[48;2;60;60;60m";
// pub const GUTTER_HIGHLIGHT: &[u8] = b"\x1b[38;2;200;200;200m\x1b[48;2;60;60;60m";
pub const GUTTER_DEFAULT: &[u8] = b"\x1b[38;2;100;100;100m";
pub const GUTTER_HIGHLIGHT: &[u8] = b"\x1b[38;2;200;200;200m";

pub const STATUS_DEFAULT: &[u8] = b"\x1b[38;2;200;200;200m\x1b[48;2;20;120;200m";

pub const TEXT_DEFAULT: &[u8] = b"\x1b[38;2;200;200;200m";

pub const HOME: &[u8] = b"\x1b[0;6H";
pub const LINE_FEED: &[u8] = b"\x1b[1E\x1b[6G";