use crate::{engine::unicode::Unicode, error::SnakeError, terminal::terminal::Terminal};

use super::gamestate::GameState;

#[derive(Debug)]
pub struct Arena {
    // x, y, character
    pub positions: Vec<(u16, u16, Unicode)>,
}

impl Arena {
    pub fn new() -> Self {
        Arena {
            positions: Vec::new(),
        }
    }

    pub fn create_level_1(mut state: GameState) -> GameState {
        let (cols, rows) = Terminal::get_console_size();

        let cols = cols / 2;
        let rows = rows / 2;

        // Todo: we need some way of tracking where other blocks have been drawn on
        // so we don't have to manually track stuff like this starting on row 3...

        // Corners
        state
            .arena
            .positions
            .push((cols - 1, 3, Unicode::BoxLightArcDownAndLeft));
        state
            .arena
            .positions
            .push((cols - 1, rows - 1, Unicode::BoxLightArcUpAndLeft));
        state
            .arena
            .positions
            .push((1, rows - 1, Unicode::BoxLightArcUpAndRight));
        state
            .arena
            .positions
            .push((1, 3, Unicode::BoxLightArcDownAndRight));

        // Top and bottom lines
        for i in 2..cols - 1 {
            state
                .arena
                .positions
                .push((i, 3, Unicode::BoxLightHorizontal));
            state
                .arena
                .positions
                .push((i, rows - 1, Unicode::BoxLightHorizontal));
        }

        // Right and left lines
        for i in 4..rows - 1 {
            state
                .arena
                .positions
                .push((cols - 1, i, Unicode::BoxLightVertical));
            state
                .arena
                .positions
                .push((1, i, Unicode::BoxLightVertical));
        }

        state
    }

    pub fn middle_coords(arena: &Arena) -> Result<(u16, u16), SnakeError> {
        let x_max = Arena::max_x(arena)?;
        let y_max = Arena::max_y(arena)?;

        let x_middle = (x_max + 1) / 2;
        let y_middle = (y_max + 1) / 2;

        Ok((x_middle, y_middle))
    }

    pub fn max_arena_coords(arena: &Arena) -> Result<(u16, u16), SnakeError> {
        let max_x = Arena::max_x(arena)?;
        let max_y = Arena::max_y(arena)?;

        Ok((max_x, max_y))
    }

    fn max_x(arena: &Arena) -> Result<u16, SnakeError> {
        // todo: remove unwraps.  Was testing. Alternative at the time was insanity
        Ok(arena.positions.iter().map(|pos| (pos.0)).max().unwrap())
    }

    fn max_y(arena: &Arena) -> Result<u16, SnakeError> {
        // todo: remove unwraps.  Was testing. Alternative at the time was insanity
        Ok(arena.positions.iter().map(|pos| (pos.1)).max().unwrap())
    }
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}
