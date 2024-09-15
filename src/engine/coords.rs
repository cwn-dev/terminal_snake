#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
}

impl Coords {
    pub fn new(x: i16, y: i16) -> Self {
        Coords { x, y }
    }

    /// Converts x and y in the given `Coords` struct to unsigned.
    /// If the value was -1 (which is expected when e.g. Snakes
    /// body parts are inactive), then x and y will be set to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use terminal_snake::state::coords::Coords;
    ///
    /// let coords = Coords { x: 5, y: 5, facing: Directions::Up };
    /// assert_eq!(coords.to_unsigned_tuple(), (5, 5));
    /// ```
    pub fn to_unsigned_tuple(&self) -> (u16, u16) {
        match (self.x, self.y) {
            (_, _) if self.x <= -1 || self.y <= -1 => (0, 0),
            // YOLO
            (_, _) => (self.x.try_into().unwrap(), self.y.try_into().unwrap()),
        }
    }

    /// Returns true if the given x and y are both greater than -1.
    ///
    /// # Examples
    ///
    /// ```
    /// use terminal_snake::engine::coords::Coords;
    ///
    /// let coords = Coords::new(-1, -1);
    /// assert_eq!(coords.is_active(), false);
    /// ```
    pub fn is_active(&self) -> bool {
        self.x > -1 && self.y > -1
    }
}

impl Default for Coords {
    fn default() -> Self {
        Coords::new(-1, -1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn to_unsigned_tuple_returns_correct_coords() {
        let coords = Coords { x: 5, y: 5 };

        assert_eq!(coords.to_unsigned_tuple(), (5, 5));
    }

    #[test]
    pub fn to_unsigned_tuple_returns_x0_y0_if_minus_1() {
        let coords = Coords { x: -1, y: -1 };

        assert_eq!(coords.to_unsigned_tuple(), (0, 0));
    }

    #[test]
    pub fn to_unsigned_tuple_returns_x0_y0_if_signed() {
        let coords = Coords { x: -42, y: -42 };

        assert_eq!(coords.to_unsigned_tuple(), (0, 0));
    }

    #[test]
    pub fn is_active_return_true_if_above_minus_1() {
        let coords = Coords::new(1, 1);
        assert_eq!(coords.is_active(), true);
    }

    #[test]
    pub fn is_active_return_true_if_0() {
        let coords = Coords::new(0, 0);
        assert_eq!(coords.is_active(), true);
    }

    #[test]
    pub fn is_active_return_false_if_minus_1() {
        let coords = Coords::new(-1, -1);
        assert_eq!(coords.is_active(), false);
    }

    #[test]
    pub fn is_active_return_false_if_below_minus_1() {
        let coords = Coords::new(-42, -44);
        assert_eq!(coords.is_active(), false);
    }
}
