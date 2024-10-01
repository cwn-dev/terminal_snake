use crate::engine::coords::Coords;

use super::arena::Arena;
use super::food::Food;
use super::snake::Snake;

#[derive(Debug)]
pub struct GameState {
    pub snake: Snake,
    pub food: Food,
    pub arena: Arena,
    pub score: i16,
    pub c_dimensions: Coords, // console dimensions
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            snake: Snake::new(),
            food: Food {
                positions: [Coords::new(-1, -1); 3],
            },
            arena: Arena::new(),
            score: 0,
            c_dimensions: Coords::new(0, 0),
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}
