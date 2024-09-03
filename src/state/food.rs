use crate::{error::SnakeError, random::random::Random, state::arena::Arena};

use super::gamestate::GameState;

#[derive(Debug)]
pub struct Food {
    pub positions: [(i16, i16); 3],
}

impl Food {
    pub fn new_random(mut state: GameState, count: i8) -> Result<GameState, SnakeError> {
        let (cols, rows) = Arena::middle_coords(&state.arena)?;
        let rand_cols = Random::time_seed().get(2, (cols - 3) as u128) as i16;
        let rand_rows = Random::time_seed().get(4, (rows - 3) as u128) as i16;

        if (count as usize) > state.food.positions.len() {
            return Err(SnakeError);
        }

        // Tried to spawn food on top of part of Snake.
        if state.snake.positions.iter().any(|&pos| (pos.x == rand_cols && pos.y == rand_rows)) {
            return Food::new_random(state, count)
        }

        state.food.positions[0] = (rand_cols as i16, rand_rows as i16);

        // Todo: move to drawing or graphics module.
        // This function should only add the position of the
        // food piece to the array. Ultimately draw the food in draw_game()
        print!("\x1b[{};{}f", rand_rows, rand_cols);
        print!("▫");

        Ok(state)
    }
}
