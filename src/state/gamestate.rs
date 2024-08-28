use super::snake::Snake;
use super::food::Food;

#[derive(Debug)]
pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub score: i16,
}
