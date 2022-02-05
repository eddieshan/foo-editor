use std::io::{Result};
use crate::core::TermInfo;

pub trait Term {
    fn restore(&self);
    fn info(&self) -> Result<TermInfo>;
}

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub fn configure() -> Result<impl Term> {
    linux::term::configure()
}

#[cfg(target_os = "windows")]
mod win;

#[cfg(target_os = "windows")]
pub fn configure() -> Result<impl Term> {
    windows::term::configure()
}