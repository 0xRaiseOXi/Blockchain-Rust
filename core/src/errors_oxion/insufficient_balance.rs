use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InsufficientBalance {
    message: String,
}

impl fmt::Display for InsufficientBalance {
    fn fmt(&selfm f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}