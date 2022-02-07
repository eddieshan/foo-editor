pub mod common;
pub mod vt100;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::term::os_configure;

#[cfg(target_os = "windows")]
mod win;
#[cfg(target_os = "windows")]
use win::term::os_configure;

use common::*;

pub fn configure() -> Result<impl Term, TermError> {
    os_configure()
}

