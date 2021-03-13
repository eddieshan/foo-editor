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