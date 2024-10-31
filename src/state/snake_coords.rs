use crate::engine::coords::Coords;

use super::directions::Directions;

#[derive(Debug, Copy, Clone)]
pub struct SnakeCoords {
    pub coords: Coords,
    pub facing: Directions,
    pub active: bool,
}

impl SnakeCoords {
    pub fn new(x: i16, y: i16, facing: Directions, active: bool) -> Self {
        SnakeCoords {
            coords: Coords::new(x, y),
            facing,
            active: active,
        }
    }
}

impl Default for SnakeCoords {
    fn default() -> Self {
        SnakeCoords::new(-1, -1, Directions::None, false)
    }
}
