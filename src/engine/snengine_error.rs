use std::{error::Error, fmt};

pub struct SnengineError {
    message: String,
}

impl Error for SnengineError {}

impl SnengineError {
    pub fn new(message: &str) -> Self {
        SnengineError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for SnengineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for SnengineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {}, message: {} }}",
            file!(),
            line!(),
            self.message
        )
    }
}
