use crate::{ error::SnakeError, random::random::Random, terminal::terminal::Terminal };

use super::gamestate::GameState;

#[derive(Debug)]
pub struct Food {
    pub positions: [(i16, i16); 3],
}

impl Food {
    pub fn new_random(mut state: GameState, count: i8) -> Result<GameState, SnakeError> {
        let (cols, rows) = Terminal::get_console_size();
        let rand_cols = Random::time_seed().get(2, (cols - 3) as u128);
        let rand_rows = Random::time_seed().get(4, (rows - 3) as u128);

        if (count as usize) > state.food.positions.len() {
            return Err(SnakeError);
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
