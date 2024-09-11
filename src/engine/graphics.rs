use super::{snengine_error::SnengineError, unicode::Unicode};

pub struct Graphics;

impl Graphics {
    pub fn draw_char(x: u16, y: u16, char: Unicode) -> Result<(), SnengineError> {
        // todo: should I implement a check to ensure we don't
        // draw outside the available terminal area?
        // Get and cache the terminal size on startup. This can be used by the
        // game & engine, which is better than having to calling get_console_size
        // all the time.

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
        if y == 0 || x == 0 {
            return Err(SnengineError::new(
                format!("Cannot write at {}, {}", x, y).as_str(),
            ));
        }

        print!("\x1b[{};{}f", y, x);
        print!("{}", text);

        Ok(())
    }

    // Todo: need to remove facing from Coords, then use Coords everywhere
    // rather than x,y tuples or vars. Then I will implement the function
    // below inside Coords so we can call Coords::valid() or something.
    // fn is_valid(&self) -> Result<bool, SnengineError> {
    //     if self.y == 0 || self.x == 0 {
    //         return Err(SnengineError::new(
    //             format!("Cannot draw at {}, {}", x, y).as_str(),
    //         ));
    //     }

    //     Ok(())
    // }
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
        let expected = Err(SnengineError::new("Cannot write at 0, 0"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn write_error_if_y_0() {
        let result = Graphics::write(42, 0, String::from("Hello"));
        let expected = Err(SnengineError::new("Cannot write at 42, 0"));
        assert_eq!(result, expected);
    }

    #[test]
    pub fn write_error_if_x_0() {
        let result = Graphics::write(0, 42, String::from("Hello"));
        let expected = Err(SnengineError::new("Cannot write at 0, 42"));
        assert_eq!(result, expected);
    }
}
