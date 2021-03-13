use std::ptr;
use std::io::{Result, Error};

use crate::win::bindings;
use crate::win::bindings::{DWORD, HANDLE, COORD, SMALL_RECT, CONSOLE_SCREEN_BUFFER_INFO};

pub struct TermState {
    std_in: (HANDLE, DWORD),
    std_out: (HANDLE, DWORD),
    pub buffer: CONSOLE_SCREEN_BUFFER_INFO
}

impl TermState {
    pub fn restore(&self) {
        set_mode(self.std_in.0, self.std_in.1);
        set_mode(self.std_out.0, self.std_out.1);
    }
}

fn get_mode(handle: HANDLE) -> Result<u32> {
    let mut console_mode = 0;
    unsafe {
        if bindings::GetConsoleMode(handle, &mut console_mode) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(console_mode)
}

fn set_mode(handle: HANDLE, console_mode: u32) -> Result<()> {
    unsafe {
        if bindings::SetConsoleMode(handle, console_mode) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

fn device_handle(device_name: &str) -> HANDLE {
    let encoded_device_name: Vec<u16> = device_name.encode_utf16().collect();

    let handle = unsafe {
        bindings::CreateFileW(
            encoded_device_name.as_ptr(),
            bindings::GENERIC_READ | bindings::GENERIC_WRITE,
            bindings::FILE_SHARE_READ | bindings::FILE_SHARE_WRITE,
            ptr::null_mut(),
            bindings::OPEN_EXISTING,
            0,
            ptr::null_mut(),
        )
    };

    handle
}

fn configure_device(device_name: &str, new_mode: fn(DWORD) -> DWORD) -> Result<(HANDLE, DWORD)> {
    let handle = device_handle(device_name);
    let current_mode = get_mode(handle)?;

    println!("CURRENT MODE {}", current_mode);

    set_mode(handle, new_mode(current_mode))?;

    Ok((handle, current_mode))
}

const RAW_INPUT_MASK: DWORD = bindings::ENABLE_LINE_INPUT |
                              bindings::ENABLE_ECHO_INPUT;

const VT_INPUT_MASK: DWORD = bindings::ENABLE_VIRTUAL_TERMINAL_INPUT |
                             bindings::ENABLE_PROCESSED_INPUT;

const CONSOLE_IN: &str = "CONIN$\0";
const CONSOLE_OUT: &str = "CONOUT$\0";

fn raw_vt_input_mode(current_mode: DWORD) -> DWORD {
    (current_mode & !RAW_INPUT_MASK) | VT_INPUT_MASK
}

fn ansi_output_mode(current_mode: DWORD) -> DWORD {
    current_mode | bindings::ENABLE_VIRTUAL_TERMINAL_PROCESSING
}

fn buffer_info(handle: HANDLE) -> Result<CONSOLE_SCREEN_BUFFER_INFO> {
    let mut buffer_info = CONSOLE_SCREEN_BUFFER_INFO  { 
        dwSize: COORD { X: 0, Y: 0 },
        dwCursorPosition: COORD { X: 0, Y: 0 },
        wAttributes: 0,
        srWindow: SMALL_RECT { Left: 0, Top: 0, Right: 0, Bottom: 0},
        dwMaximumWindowSize: COORD { X: 0, Y: 0 }
    };

    unsafe {
        if bindings::GetConsoleScreenBufferInfo(handle, &mut buffer_info) == 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(buffer_info)
}

pub fn configure() -> Result<TermState> {
    let std_in = configure_device(CONSOLE_IN, raw_vt_input_mode)?;
    let std_out = configure_device(CONSOLE_OUT, ansi_output_mode)?;
    let buffer = buffer_info(std_out.0)?;

    Ok(TermState {
        std_in: std_in,
        std_out: std_out,
        buffer: buffer
    })
}