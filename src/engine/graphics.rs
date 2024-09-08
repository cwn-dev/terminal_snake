use super::{snengine_error::SnengineError, unicode::Unicode};

pub struct Graphics;

impl Graphics {
    pub fn draw_char(x: u16, y: u16, char: Unicode) -> Result<(), SnengineError> {
        // todo: should I implement a check to ensure we don't
        // draw outside the available terminal area?

        if y == 0 || x == 0 {
            return Err(SnengineError::new(
                format!("Cannot draw at {}, {}", x, y).as_str(),
            ));
        }

        print!("\x1b[{};{}f", y, x);
        print!("{}", char.to_char());

        Ok(())
    }

    pub fn write(x: u16, y: u16, text: String) -> Result<(), SnengineError> {
        // Todo: expand this a bit. Need the x & y = 0 check,
        // but don't want to have to repeat myself. Will
        // look later.
        print!("\x1b[{};{}f", y, x);
        print!("{}", text);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Todo: SnengineError should be an enum, or have some
    // enum to go with it so the test below will work.

    //use super::*;

    // #[test]
    // pub fn draw_char_error_if_x_0() {
    //     let result = Graphics::draw_char(0, 42, Unicode::Space);
    //     let expected = Err(SnengineError::ErrorType);
    //     assert_eq!(result, expected);
    // }
}