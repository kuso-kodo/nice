use std::fmt;

cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::set_thread_priority;
        #[derive(Debug)]
        pub struct OsError(pub(crate) u32);
    } else {
        mod unix;
        pub use unix::set_thread_priority;
        #[derive(Debug)]
        pub struct OsError(pub(crate) i32);
    }
}

const PRIORITY_MAX: u8 = 100;

pub(crate) type Result<T> = std::result::Result<T, OsError>;

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "os error: {}", self.0)
    }
}

impl std::error::Error for OsError {}

#[derive(Debug, Clone, Copy)]
pub struct Priority(u8);

impl From<u8> for Priority {
    fn from(value: u8) -> Priority {
        Priority(value.min(PRIORITY_MAX))
    }
}
