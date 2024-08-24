#[derive(Debug, Clone, Copy)]
pub enum Directions {
    None,
    Up,
    Down,
    Right,
    Left
}

impl PartialEq for Directions {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}