#[derive(Debug, Clone, Copy)]
pub enum Directions {
    None,
    Up,
    Down,
    Right,
    Left,
}

impl PartialEq for Directions {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Directions::None, Directions::None) => true,
            (Directions::Down, Directions::Down) => true,
            (Directions::Left, Directions::Left) => true,
            (Directions::Right, Directions::Right) => true,
            (Directions::Up, Directions::Up) => true,
            _ => false,
        }
    }
}
