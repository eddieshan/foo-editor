use std::os::raw::*;

pub type wchar_t = u16;

pub type BOOL = c_int;
pub type SHORT = c_short;
pub type WORD = c_ushort;
pub type DWORD = c_ulong;
pub type LPDWORD = *mut DWORD;
pub type HANDLE = *mut c_void;
pub type LPVOID = *mut c_void;
pub type WCHAR = wchar_t;
pub type LPCWSTR = *const WCHAR;

#[repr(C)]
pub struct SECURITY_ATTRIBUTES {
    nLength: DWORD,
    lpSecurityDescriptor: LPVOID,
    bInheritHandle: BOOL
}

pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;

#[repr(C)]
pub struct COORD {
    pub X: SHORT,
    pub Y: SHORT
}

#[repr(C)]
pub struct SMALL_RECT {
    pub Left: SHORT,
    pub Top: SHORT,
    pub Right: SHORT,
    pub Bottom: SHORT
}

#[repr(C)]
pub struct CONSOLE_SCREEN_BUFFER_INFO {
    pub dwSize: COORD,
    pub dwCursorPosition: COORD,
    pub wAttributes: WORD,
    pub srWindow: SMALL_RECT,
    pub dwMaximumWindowSize: COORD
}

pub type PCONSOLE_SCREEN_BUFFER_INFO = *mut CONSOLE_SCREEN_BUFFER_INFO;

pub const ENABLE_PROCESSED_INPUT: DWORD        = 0x0001;
pub const ENABLE_LINE_INPUT: DWORD             = 0x0002;
pub const ENABLE_ECHO_INPUT: DWORD             = 0x0004;
pub const ENABLE_VIRTUAL_TERMINAL_INPUT: DWORD = 0x0200;

pub const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;

pub const GENERIC_READ: DWORD       = 0x80000000;
pub const GENERIC_WRITE: DWORD      = 0x40000000;

pub const FILE_SHARE_READ: DWORD    = 0x00000001;
pub const FILE_SHARE_WRITE: DWORD   = 0x00000002;

pub const OPEN_EXISTING: DWORD = 3;