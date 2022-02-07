use std::ptr;
use std::convert::TryFrom;

use crate::term::common::*;
use crate::term::win::bindings::*;

// This is annoying. The trait should be just From instead of TryFrom.
// But only in Win32 you will see window dimensions measured with signed short.
// It's perfectly logical to have negative width and height right? Well done, MS.
impl TryFrom<COORD> for Size {
    type Error = std::num::TryFromIntError;
    fn try_from(coord: COORD) -> std::result::Result<Self, Self::Error> {
        Ok(Size { 
            width: usize::try_from(coord.X)?, 
            height: usize::try_from(coord.Y)?
        })
    } 
}

// A pair of: 
// - handle to a term stream,
// - the original attributes for that stream.
pub type StreamState = (HANDLE, DWORD);

// Stores the state for input and output streams.
pub struct WinTerm {
    std_in: StreamState,
    std_out: StreamState
}

impl Term for WinTerm {
    fn restore(&self) -> Result<(), TermError> {
        set_mode(self.std_in.0, self.std_in.1)?;
        set_mode(self.std_out.0, self.std_out.1)
    }

    fn info(&self) -> Result<TermInfo, TermError> {
        let mut buffer_info = CONSOLE_SCREEN_BUFFER_INFO  { 
            dwSize: COORD { X: 0, Y: 0 },
            dwCursorPosition: COORD { X: 0, Y: 0 },
            wAttributes: 0,
            srWindow: SMALL_RECT { Left: 0, Top: 0, Right: 0, Bottom: 0},
            dwMaximumWindowSize: COORD { X: 0, Y: 0 }
        };
    
        unsafe {
            if GetConsoleScreenBufferInfo(self.std_out.0, &mut buffer_info) == 0 {
                return Err(TermError::CannotGetTermAttributes);
            }
        }

        match (Size::try_from(buffer_info.dwSize), Size::try_from(buffer_info.dwMaximumWindowSize)) {
            (Ok(buffer_size), Ok(window_size)) => Ok(TermInfo { buffer_size: buffer_size, screen_size: window_size }),
            _ => Err(TermError::InvalidTermAttributes)
        }
    }
}

fn get_mode(handle: HANDLE) -> Result<DWORD, TermError> {
    let mut console_mode = 0;

    unsafe {
        match GetConsoleMode(handle, &mut console_mode)  {
            0 => Err(TermError::CannotGetTermAttributes),
            _ => Ok(console_mode)
        }
    }
}

fn set_mode(handle: HANDLE, console_mode: DWORD) -> Result<(), TermError> {
    unsafe {
        match SetConsoleMode(handle, console_mode) {
            0 => Err(TermError::CannotSetTermAttributes),
            _ => Ok(())
        }
    }
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

fn configure_device(device_name: &str, new_mode: fn(DWORD) -> DWORD) -> Result<StreamState, TermError> {
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

pub fn os_configure() -> Result<impl Term, TermError> {   
    let std_in = configure_device(CONSOLE_IN, raw_vt_input_mode)?;
    let std_out = configure_device(CONSOLE_OUT, ansi_output_mode)?;

    Ok(WinTerm {
        std_in: std_in,
        std_out: std_out
    })
}