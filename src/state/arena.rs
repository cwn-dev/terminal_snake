use crate::{error::SnakeError, terminal::terminal::Terminal};

use super::gamestate::GameState;

#[derive(Debug)]
pub struct Arena {
    // x, y, character
    pub positions: Vec<(u16, u16, String)>,
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
        state.arena.positions.push((cols - 1, 3, String::from("╮")));
        state.arena.positions.push((cols - 1, rows - 1, String::from("╯")));
        state.arena.positions.push((1, rows - 1, String::from("╰")));
        state.arena.positions.push((1, 3, String::from("╭")));

        // Top and bottom lines
        for i in 2..cols - 1 {
            state.arena.positions.push((i, 3, String::from("─")));
            state.arena.positions.push((i, rows - 1, String::from("─")));
        }

        // Right and left lines
        for i in 4..rows - 1 {
            state.arena.positions.push((cols - 1, i, String::from("│")));
            state.arena.positions.push((1, i, String::from("│")));
        }

        state
    }

    pub fn middle_coords(arena: &Arena) -> Result<(u16, u16), SnakeError> {
        // todo: remove unwraps.  Was testing. Alternative at the time was insanity
        let x_max = arena.positions
            .iter()
            .map(|pos| (pos.0))
            .max()
            .unwrap();

        let y_max = arena.positions
            .iter()
            .map(|pos| (pos.1))
            .max()
            .unwrap();

        let x_middle = (x_max + 1) / 2;
        let y_middle = (y_max + 1) / 2;

        Ok((x_middle, y_middle))
    }
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}