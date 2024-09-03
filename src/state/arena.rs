use crate::terminal::terminal::Terminal;

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
}

impl Default for Arena {
    fn default() -> Self {
        Arena::new()
    }
}
