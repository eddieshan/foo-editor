use std::ptr;
use std::convert::TryFrom;
use std::io::{Result, Error};

use crate::core::*;
use crate::term::Term;
use crate::term::win::bindings::*;

impl TryFrom<COORD> for Size {
    type Error = Error;
    fn try_from(coord: COORD) -> std::result::Result<Self, Self::Error> {
        match (usize::try_from(coord.X), usize::try_from(coord.Y)) {
            (Ok(w), Ok(h)) => Ok(Size { width: w, height: h }),
            _              => Err(Error::last_os_error())
        }
    } 
}

// Stores the handles for input and output channels and a backup of 
// the original console modes for each of them
pub struct WinTerm {
    std_in: (HANDLE, DWORD),
    std_out: (HANDLE, DWORD)
}

impl Term for WinTerm {
    fn restore(&self) {
        set_mode(self.std_in.0, self.std_in.1);
        set_mode(self.std_out.0, self.std_out.1);
    }

    fn info(&self) -> Result<TermInfo> {
        let mut buffer_info = CONSOLE_SCREEN_BUFFER_INFO  { 
            dwSize: COORD { X: 0, Y: 0 },
            dwCursorPosition: COORD { X: 0, Y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT { Left: 0, Top: 0, Right: 0, Bottom: 0},
            dwMaximumWindowSize: COORD { X: 0, Y: 0 }
        };
    
        unsafe {
            if GetConsoleScreenBufferInfo(self.std_out.0, &mut buffer_info) == 0 {
                return Err(Error::last_os_error());
            }
        }
    
        Ok(TermInfo {
            buffer_size: Size::try_from(buffer_info.dwSize)?,
            screen_size: Size::try_from(buffer_info.dwMaximumWindowSize)?
        })
    }
}

fn get_mode(handle: HANDLE) -> Result<DWORD> {
    let mut console_mode = 0;
    unsafe {
        if GetConsoleMode(handle, &mut console_mode) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(console_mode)
}

fn set_mode(handle: HANDLE, console_mode: DWORD) -> Result<()> {
    unsafe {
        if SetConsoleMode(handle, console_mode) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

fn device_handle(device_name: &str) -> HANDLE {
    let encoded_device_name: Vec<u16> = device_name.encode_utf16().collect();

    let handle = unsafe {
        CreateFileW(
            encoded_device_name.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            ptr::null_mut(),
            OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };

    handle
}

fn configure_device(device_name: &str, new_mode: fn(DWORD) -> DWORD) -> Result<(HANDLE, DWORD)> {
    let handle = device_handle(device_name);
    let current_mode = get_mode(handle)?;

    set_mode(handle, new_mode(current_mode))?;

    Ok((handle, current_mode))
}

const RAW_INPUT_MASK: DWORD = ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT;
const VT_INPUT_MASK: DWORD = ENABLE_VIRTUAL_TERMINAL_INPUT | ENABLE_PROCESSED_INPUT;

fn raw_vt_input_mode(current_mode: DWORD) -> DWORD {
    (current_mode & !RAW_INPUT_MASK) | VT_INPUT_MASK
}

fn ansi_output_mode(current_mode: DWORD) -> DWORD {
    current_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING
}

const CONSOLE_IN: &str = "CONIN$\0";
const CONSOLE_OUT: &str = "CONOUT$\0";

pub fn configure() -> Result<WinTerm> {   
    let std_in = configure_device(CONSOLE_IN, raw_vt_input_mode)?;
    let std_out = configure_device(CONSOLE_OUT, ansi_output_mode)?;

    Ok(WinTerm {
        std_in: std_in,
        std_out: std_out
    })
}