use super::food::Food;
use super::snake::Snake;

#[derive(Debug)]
pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub score: i16,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            score: 0,
            snake: Snake::new(),
            food: Food {
                positions: [(-1, -1); 3],
            },
        }
    }
}
