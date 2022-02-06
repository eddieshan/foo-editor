use std::io::{Result};
use crate::core::geometry::Size;

pub struct TermInfo {
    pub buffer_size: Size,
    pub screen_size: Size
}

pub trait Term {
    fn restore(&self) -> Result<()>;
    fn info(&self) -> Result<TermInfo>;
}

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::term::os_configure;

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
use win::term::os_configure;

pub fn configure() -> Result<impl Term> {    
    os_configure()
}

pub mod vt100;