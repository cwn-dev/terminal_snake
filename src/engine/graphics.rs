use super::{snengine_error::SnengineError, unicode_character::UnicodeCharacter};

pub struct Graphics;

impl Graphics {
    pub fn draw_char(x: i16, y: i16, char: UnicodeCharacter) -> Result<(), SnengineError> {
        if y == -1 || x == -1 {
            return Err(SnengineError::new(
                format!("Cannot draw at {}, {}", x, y).as_str(),
            ));
        }

        // todo: should I implement a check to ensure we don't
        // draw outside the available terminal area?

        print!("\x1b[{};{}f", y, x);
        print!("{}", char.to_char());

        Ok(())
    }
}
