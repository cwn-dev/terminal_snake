extern crate libc;

use libc::{TCSANOW, tcsetattr, STDIN_FILENO};
use std::thread;
use std::time::Duration;
use std::fs::File;

use state::gamestate::GameState;
use state::coords::Coords;
use state::directions::Directions;
use state::snake::Snake;

use terminal::terminal::Terminal;
use engine::inputhandler::InputHandler;

pub mod state;
pub mod terminal;
pub mod engine;

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
        state = InputHandler::handle_input(state, &file);
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