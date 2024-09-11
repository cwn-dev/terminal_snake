use crate::{
    engine::coords::Coords, error::SnakeError, random::random::Random, state::arena::Arena,
};

use super::gamestate::GameState;

#[derive(Debug)]
pub struct Food {
    pub positions: [Coords; 3],
}

impl Food {
    pub fn new_random(mut state: GameState, count: i8) -> Result<GameState, SnakeError> {
        let (cols, rows) = Arena::max_arena_coords(&state.arena)?;
        let rand_cols = Random::time_seed().get(2, (cols - 1).into()) as i16;
        let rand_rows = Random::time_seed().get(4, (rows - 1).into()) as i16;

        if (count as usize) > state.food.positions.len() {
            return Err(SnakeError);
        }

        // Tried to spawn food on top of part of Snake.
        if state
            .snake
            .positions
            .iter()
            .any(|&pos| (pos.coords.x == rand_cols && pos.coords.y == rand_rows))
        {
            return Food::new_random(state, count);
        }

        state.food.positions[0].x = rand_cols;
        state.food.positions[0].y = rand_rows;

        Ok(state)
    }
}
