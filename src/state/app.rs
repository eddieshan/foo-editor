use crate::core::geometry::*;
use crate::text::region::*;
use super::text_area::*;

pub struct AppState {
    pub region: Region,
    pub text_area: TextArea
}

impl AppState {
    pub fn new(window: Size) -> Self {
        AppState {
            region: Region { 
                start: 0, 
                page_size: window.height - 1 
            },
            text_area: TextArea::new(window)
        }
    }
}