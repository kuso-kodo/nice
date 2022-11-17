//! 在 windows 平台，使用 [`SetThreadPriority`] 设置优先级。
//!
//! - Ref: <https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setthreadpriority>

use windows_sys::Win32::{
    Foundation::GetLastError,
    System::Threading::{
        GetCurrentThread, SetThreadPriority, THREAD_PRIORITY, THREAD_PRIORITY_ABOVE_NORMAL,
        THREAD_PRIORITY_BELOW_NORMAL, THREAD_PRIORITY_HIGHEST, THREAD_PRIORITY_IDLE,
        THREAD_PRIORITY_LOWEST, THREAD_PRIORITY_NORMAL, THREAD_PRIORITY_TIME_CRITICAL,
    },
};

use crate::{OsError, Priority, Result};

impl Priority {
    fn to_windows(self) -> THREAD_PRIORITY {
        match self.0 {
            0 => THREAD_PRIORITY_IDLE,
            1..=19 => THREAD_PRIORITY_LOWEST,
            20..=39 => THREAD_PRIORITY_BELOW_NORMAL,
            40..=59 => THREAD_PRIORITY_NORMAL,
            60..=79 => THREAD_PRIORITY_ABOVE_NORMAL,
            80..=99 => THREAD_PRIORITY_HIGHEST,
            _ => THREAD_PRIORITY_TIME_CRITICAL,
        }
    }
}

pub fn set_thread_priority(priority: Priority) -> Result<()> {
    unsafe {
        let handle = GetCurrentThread();
        if SetThreadPriority(handle, priority.to_windows()) == 0 {
            Err(OsError(GetLastError()))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_windows() -> Result<()> {
        for i in 0..114 {
            let priority = Priority::from(i).to_windows();
            if i == 0 {
                assert_eq!(priority, THREAD_PRIORITY_IDLE);
            } else if i < 20 {
                assert_eq!(priority, THREAD_PRIORITY_LOWEST);
            } else if i < 40 {
                assert_eq!(priority, THREAD_PRIORITY_BELOW_NORMAL);
            } else if i < 60 {
                assert_eq!(priority, THREAD_PRIORITY_NORMAL);
            } else if i < 80 {
                assert_eq!(priority, THREAD_PRIORITY_ABOVE_NORMAL);
            } else if i < 100 {
                assert_eq!(priority, THREAD_PRIORITY_HIGHEST);
            } else {
                assert_eq!(priority, THREAD_PRIORITY_TIME_CRITICAL);
            }
        }

        set_thread_priority(Priority::from(50)).unwrap();
        set_thread_priority(Priority::from(90)).unwrap();
        set_thread_priority(Priority::from(40)).unwrap();
    }
}
