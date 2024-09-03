use std::fs::File;
use std::io::Read;

use crate::Directions;
use crate::GameState;

pub struct InputHandler {}

impl InputHandler {
    pub fn handle_input(mut state: GameState, mut file: &File) -> GameState {
        let mut buffer = [0; 3]; // Buffer to store input characters

        match file.read(&mut buffer) {
            Ok(0) => state,
            Ok(_) => {
                // Left Arrow
                if buffer[0] == 0x1b && buffer[1] == 0x5b && buffer[2] == 0x44 {
                    state.snake.direction = Directions::Left;
                }

                // Right Arrow
                if buffer[0] == 0x1b && buffer[1] == 0x5b && buffer[2] == 0x43 {
                    state.snake.direction = Directions::Right;
                }

                // Up Arrow
                if buffer[0] == 0x1b && buffer[1] == 0x5b && buffer[2] == 0x41 {
                    state.snake.direction = Directions::Up;
                }

                // Down Arrow
                if buffer[0] == 0x1b && buffer[1] == 0x5b && buffer[2] == 0x42 {
                    state.snake.direction = Directions::Down;
                }

                state
            }
            Err(_) => state,
        }
    }
}
