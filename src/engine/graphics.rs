use super::{snengine_error::SnengineError, unicode_character::UnicodeCharacter};

pub struct Graphics {
    x: i16,
    y: i16,
    char: UnicodeCharacter,
}

impl Graphics {
    pub fn draw_char(&self) -> Result<(), SnengineError> {
        if self.y == -1 || self.x == -1 {
            return Err(SnengineError::new(
                format!("Cannot draw at {}, {}", self.x, self.y).as_str(),
            ));
        }

        // todo: should I implement a check to ensure we don't
        // draw outside the available terminal area?

        print!("\x1b[{};{}f", self.y, self.x);
        print!("{}", self.char.to_char());

        Ok(())
    }
}
