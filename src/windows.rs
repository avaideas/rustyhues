#[cfg(windows)]
use std::sync::OnceLock;
#[cfg(windows)]
use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::FILE_TYPE_CHAR;
#[cfg(windows)]
use windows_sys::Win32::System::Console::*;

#[cfg(windows)]
static STDOUT_VT: OnceLock<bool> = OnceLock::new();
#[cfg(windows)]
static STDERR_VT: OnceLock<bool> = OnceLock::new();
#[cfg(windows)]
pub fn stdout_vt_enabled() -> bool {
    *STDOUT_VT.get_or_init(|| enable_vt_for_stdout());
}
#[cfg(windows)]
pub fn stderr_vt_enabled() -> bool {
    *STDERR_VT.get_or_init(|| enable_vt_for_stderr());
}

#[cfg(windows)]
fn std_handle(which: u32) -> isize {
    unsafe { GetStdHandle(which) }
}

#[cfg(windows)]
fn is_console_handle(handle: isize) -> bool {
    if handle == 0 || handle == INVALID_HANDLE_VALUE as isize {
        return false;
    }
    let t = unsafe { GetFileType(handle) };
    t == FILE_TYPE_CHAR
}

#[cfg(windows)]
fn enable_vt(handle: isize) -> bool {
    let mut mode: u32 = 0;
    let ok = unsafe { GetConsoleMode(handle, &mut mode as *mut u32) };
    if ok == 0 {
        return false;
    }

    let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
    let ok2 = unsafe { SetConsoleMode(handle, new_mode) };
    ok2 != 0
}

#[cfg(windows)]
pub fn is_tty_stdout() -> bool {
    let handle = std_handle(STD_OUTPUT_HANDLE);
    if !is_console_handle(handle) {
        return false;
    }
    stdout_vt_enabled()
}

#[cfg(windows)]
pub fn is_tty_stderr() -> bool {
    let handle = std_handle(STD_ERROR_HANDLE);
    if !is_console_handle(handle) {
        return false;
    }
    stderr_vt_enabled()
}
