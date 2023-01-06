use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct StatusError {
    message: String,
}

impl StatusError {
    pub fn new(message: &str) -> StatusError {
        StatusError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for StatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for StatusError {}
