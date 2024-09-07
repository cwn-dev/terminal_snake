use std::fmt;

use crate::engine::snengine_error::SnengineError;

pub enum SnakeError {
    SnengineError(Option<SnengineError>), // Todo: unsure about Option in here but it makes for an easier map in draw_snake
}

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
