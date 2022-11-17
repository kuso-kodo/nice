#[cfg(any(target_os = "macos", target_os = "ios"))]
mod apple;
#[cfg(any(target_os = "linux", target_os = "android"))]
mod linux;

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use apple::set_thread_priority;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use linux::set_thread_priority;

use crate::{OsError, Result};

#[inline]
fn call_libc_set_errno<T, F: FnOnce() -> T>(f: F) -> Result<T> {
    set_errno(0);
    let result = f();
    match errno() {
        0 => Ok(result),
        errno => Err(OsError(errno)),
    }
}

#[inline]
fn call_libc_ret_errno<F: FnOnce() -> libc::c_int>(f: F) -> Result<()> {
    let result = f();
    match result {
        0 => Ok(()),
        errno => Err(OsError(errno)),
    }
}

fn errno() -> libc::c_int {
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "android")] {
                *libc::__errno()
            } else if #[cfg(target_os = "linux")] {
                *libc::__errno_location()
            } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
                *libc::__error()
            } else {
                compile_error!("Your OS is probably not supported.")
            }
        }
    }
}

fn set_errno(number: libc::c_int) {
    unsafe {
        cfg_if::cfg_if! {
            if #[cfg(target_os = "android")] {
                *libc::__errno() = number;
            } else if #[cfg(target_os = "linux")] {
                *libc::__errno_location() = number;
            } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
                *libc::__error() = number;
            } else {
                compile_error!("Your OS is probably not supported.")
            }
        }
    }
}
