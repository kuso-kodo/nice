//! 在 Linux/Android 平台，使用 [`setpriority`] 设置优先级。
//!
//! - Ref: <https://linux.die.net/man/2/setpriority>

use super::call_libc_set_errno;
use crate::{Priority, Result, PRIORITY_MAX};

const NICE_MAX: i32 = 19;
const NICE_MIN: i32 = -20;

impl Priority {
    fn to_nice(self) -> libc::c_int {
        let offset = (NICE_MAX - NICE_MIN) * (self.0 as i32) / (PRIORITY_MAX as i32);
        NICE_MAX - offset
    }
}

pub fn set_thread_priority(priority: Priority) -> Result<()> {
    call_libc_set_errno(|| unsafe {
        libc::setpriority(libc::PRIO_PROCESS, 0, priority.to_nice());
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_nice() {
        assert_eq!(Priority::from(0).to_nice(), 19);
        assert_eq!(Priority::from(100).to_nice(), -20);
        // 优先级越高，nice 越小
        assert!(Priority::from(50).to_nice() > Priority::from(70).to_nice());
        set_thread_priority(Priority::from(30)).unwrap();
        set_thread_priority(Priority::from(10)).unwrap();
    }
}
