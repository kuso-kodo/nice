//! 在 macOS/iOS 平台，使用 [`pthread_setschedparam`] 设置优先级。
//!
//! - Ref: <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/Multithreading/CreatingThreads/CreatingThreads.html#Setting%20the%20Thread%20Priority>

use std::mem::MaybeUninit;

use crate::Priority;
use crate::Result;
use crate::PRIORITY_MAX;

use super::call_libc_ret_errno;
use super::call_libc_set_errno;

type ThreadId = libc::pthread_t;

fn thread_id() -> ThreadId {
    unsafe { libc::pthread_self() }
}

impl Priority {
    fn to_posix(self, min_priority: libc::c_int, max_priority: libc::c_int) -> libc::c_int {
        let offset = (max_priority - min_priority) * (self.0 as i32) / (PRIORITY_MAX as i32);
        min_priority + offset
    }
}

pub fn set_thread_priority(priority: Priority) -> Result<()> {
    let id = thread_id();

    let mut policy = 0i32;
    let mut params = unsafe { MaybeUninit::<libc::sched_param>::zeroed().assume_init() };
    call_libc_ret_errno(|| unsafe {
        libc::pthread_getschedparam(id, &mut policy as _, &mut params as _)
    })?;

    let min_priority = call_libc_set_errno(|| unsafe { libc::sched_get_priority_min(policy) })?;
    let max_priority = call_libc_set_errno(|| unsafe { libc::sched_get_priority_max(policy) })?;

    params.sched_priority = priority.to_posix(min_priority, max_priority);

    call_libc_ret_errno(|| unsafe { libc::pthread_setschedparam(id, policy, &params as _) })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_posix() -> Result<()> {
        assert!(Priority::from(50).to_posix(0, 100) < Priority::from(70).to_posix(0, 100));
        set_thread_priority(Priority::from(50)).unwrap();
        set_thread_priority(Priority::from(90)).unwrap();
        set_thread_priority(Priority::from(40)).unwrap();
        Ok(())
    }
}
