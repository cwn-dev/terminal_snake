use super::directions::Directions;

#[derive(Debug, Copy, Clone)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
    // Todo: I'm not sure anymore if we should have directions in here
    // If we move it, then we can use Coords everywhere rather than the tuples.
    // We probably need another struct which has Coords + facing which is
    // only used for Snake.
    pub facing: Directions,
}

impl Coords {
    fn new(x: i16, y: i16, facing: Directions) -> Self {
        Coords { x, y, facing }
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
}

impl Default for Coords {
    fn default() -> Self {
        Coords::new(-1, -1, Directions::None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn to_unsigned_tuple_returns_correct_coords() {
        let coords = Coords {
            x: 5,
            y: 5,
            facing: Directions::Up,
        };
        assert_eq!(coords.to_unsigned_tuple(), (5, 5));
    }

    #[test]
    pub fn to_unsigned_tuple_returns_x0_y0_if_minus_1() {
        let coords = Coords {
            x: -1,
            y: -1,
            facing: Directions::Up,
        };
        assert_eq!(coords.to_unsigned_tuple(), (0, 0));
    }

    #[test]
    pub fn to_unsigned_tuple_returns_x0_y0_if_signed() {
        let coords = Coords {
            x: -42,
            y: -42,
            facing: Directions::Up,
        };
        assert_eq!(coords.to_unsigned_tuple(), (0, 0));
    }
}
