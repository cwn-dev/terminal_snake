use super::coords::Coords;
use super::directions::Directions;
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
            snake: Snake {
                positions: [Coords {
                    x: -1,
                    y: -1,
                    facing: Directions::None,
                }; 20],
                direction: Directions::None,
            },
            food: Food {
                positions: [(-1, -1); 3],
            },
        }
    }
}
