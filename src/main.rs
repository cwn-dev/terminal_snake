extern crate libc;

use libc::{TCSANOW, tcsetattr, STDIN_FILENO};
use state::coords::Coords;
use terminal::terminal::Terminal;
use std::io::Read;
use std::thread;
use std::time::Duration;
use std::fs::File;

use state::gamestate::GameState;
use state::directions::Directions;
use state::snake::Snake;

pub mod state;
pub mod terminal;

fn handle_input(mut state: GameState, mut file: &File) -> GameState {
    let mut buffer = [0; 3]; // Buffer to store input characters

    match file.read(&mut buffer) {
        Ok(0) => { 
            state
        },
        Ok(_) => {
            // Left Arrow
            if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x44 {
                state.snake.direction = Directions::Left;
            }

            // Right Arrow
            if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x43 {
                state.snake.direction = Directions::Right;
            }

            // Up Arrow
            if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x41 {        
                state.snake.direction = Directions::Up;
            }

            // Down Arrow
            if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x42 {
                state.snake.direction = Directions::Down;
            }

            state
        },
        Err(_) => {
            return state
        }
    }
}

fn draw_snake(mut state: GameState) -> GameState {
    // If snake's head is at -1, -1 then this is a new game, so put snake in the middle
    if state.snake.positions[0].x == -1 || state.snake.positions[0].y == -1 {
        let (cols, rows) = Terminal::get_console_size();
        let middle_x = (cols + 1) / 2;
        let middle_y = (rows + 1) / 2;

        state.snake.positions[0].x = middle_x as i16;
        state.snake.positions[0].y = middle_y as i16;

        return state
    }

    let snake_head_pos = &state.snake.positions[0];
    print!("\x1b[{};{}f", snake_head_pos.y, snake_head_pos.x);
    
    // Clear the old snake
    for p in state.snake.positions.iter() {
        print!("\x1b[{};{}f", p.y, p.x);
        print!(" ");
    }

    state.snake.step();

    // Draw the new snake
    for p in state.snake.positions.iter() {
        if p.x == -1 || p.y == -1 {
            continue;
        }

        print!("\x1b[{};{}f", p.y, p.x);

        match p.facing {
            Directions::Down => println!("║"),
            Directions::Up =>  println!("║"),
            Directions::Left => println!("═"),
            Directions::Right => println!("═"),
            _ => println!("║")
        }
    }

    // Todo: implement debug mode so we can see stuff like this in a bar at the bottom
    //println!("{:?}", &state);

    state
}

fn game_loop(file: File) {
    let mut state = GameState {
        snake: Snake {
            positions: [Coords { x: -1, y: -1, facing: Directions::None }; 20],
            direction: Directions::None
        },
    };

    loop {
        state = handle_input(state, &file);
        state = draw_snake(state);

        //thread::sleep(Duration::from_millis(16)); // about 60 fps
        thread::sleep(Duration::from_millis(500));
    }
}

fn main() {
    let original_term = Terminal::set_raw_mode();
    let file = Terminal::set_non_blocking_stdin();

    // Clear the screen
    print!("\x1b[2J");

    // Move to 0
    print!("\x1b[H");

    // Hide the cursor
    print!("\x1b[?25l");

    println!("Welcome to terminal_snake");
    println!("Press any key to start");
    
    game_loop(file);

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}