use std::{error::Error, fmt};

pub struct SnakeError;

impl Error for SnakeError {}

impl fmt::Display for SnakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Something broke.")
    }
}

impl fmt::Debug for SnakeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!())
    }
}
