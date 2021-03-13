use std::ptr;
use std::io::{Result, Error};

use crate::win::bindings;
use crate::win::bindings::{DWORD, HANDLE, COORD, SMALL_RECT, CONSOLE_SCREEN_BUFFER_INFO};

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