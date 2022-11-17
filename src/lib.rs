use std::fmt;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod unix;

pub const PRIORITY_MAX: u8 = 100;

pub(crate) type Result<T> = std::result::Result<T, OsError>;

#[derive(Debug)]
pub struct OsError(pub(crate) i32);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
