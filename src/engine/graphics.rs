use super::{snengine_error::SnengineError, unicode::Unicode};

pub struct Graphics;

impl Graphics {
    pub fn draw_char(x: u16, y: u16, char: Unicode) -> Result<(), SnengineError> {
        // todo: should I implement a check to ensure we don't
        // draw outside the available terminal area?

        print!("\x1b[{};{}f", y, x);
        print!("{}", char.to_char());

        Ok(())
    }
}
