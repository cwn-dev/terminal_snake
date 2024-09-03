use super::arena::Arena;
use super::food::Food;
use super::snake::Snake;

#[derive(Debug)]
pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub arena: Arena,
    pub score: i16,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            snake: Snake::new(),
            food: Food {
                positions: [(-1, -1); 3],
            },
            arena: Arena::new(),
            score: 0,
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
