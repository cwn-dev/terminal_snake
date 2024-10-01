use super::{snengine_error::SnengineError, unicode::Unicode};

pub struct Graphics;

impl Graphics {
    pub fn draw_char(x: u16, y: u16, char: Unicode) -> Result<(), SnengineError> {
        // todo: should I implement a check to ensure we don't
        // draw outside the available terminal area?
        // Best solution to this I think would be to have a
        // function which runs on startup of the game and
        // checks the terminal size to make sure it can
        // run the game + it can store the terminal size
        // in memory somewhere, then we can put the check in.

        if Graphics::is_valid(x, y)? {
            print!("\x1b[{};{}f", y, x);
            print!("{}", char.to_char());
        }

        Ok(())
    }

    pub fn write(x: u16, y: u16, text: String) -> Result<(), SnengineError> {
        if Graphics::is_valid(x, y)? {
            print!("\x1b[{};{}f", y, x);
            print!("{}", text);
        }

        Ok(())
    }

    fn is_valid(x: u16, y: u16) -> Result<bool, SnengineError> {
        if x == 0 || y == 0 {
            return Err(SnengineError::new(
                format!("Cannot draw at {}, {}", x, y).as_str(),
            ));
        }

        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn draw_char_error_if_x_0() {
        let result = Graphics::draw_char(0, 42, Unicode::Space);
        let expected = Err(SnengineError::new("Cannot draw at 0, 42"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn draw_char_error_if_y_0() {
        let result = Graphics::draw_char(42, 0, Unicode::Space);
        let expected = Err(SnengineError::new("Cannot draw at 42, 0"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn draw_char_error_if_x_and_y_0() {
        let result = Graphics::draw_char(0, 0, Unicode::Space);
        let expected = Err(SnengineError::new("Cannot draw at 0, 0"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn write_error_if_x_and_y_0() {
        let result = Graphics::write(0, 0, String::from("Hello"));
        let expected = Err(SnengineError::new("Cannot draw at 0, 0"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn write_error_if_y_0() {
        let result = Graphics::write(42, 0, String::from("Hello"));
        let expected = Err(SnengineError::new("Cannot draw at 42, 0"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn write_error_if_x_0() {
        let result = Graphics::write(0, 42, String::from("Hello"));
        let expected = Err(SnengineError::new("Cannot draw at 0, 42"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn is_valid_return_true_if_x_and_y_not_0() {
        let result = Graphics::is_valid(1, 1);
        let expected = Ok(true);
        assert_eq!(result, expected);
    }

    #[test]
    pub fn is_valid_return_false_if_x_0() {
        let result = Graphics::is_valid(0, 1);
        let expected = Err(SnengineError::new("Cannot draw at 0, 1"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn is_valid_return_false_if_y_0() {
        let result = Graphics::is_valid(1, 0);
        let expected = Err(SnengineError::new("Cannot draw at 1, 0"));
        assert_eq!(result, expected);
    }
}
