use super::directions::Directions;

#[derive(Debug, Copy, Clone)]
pub struct Coords {
    pub x: i16,
    pub y: i16,
    pub facing: Directions,
}
