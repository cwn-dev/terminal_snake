extern crate libc;

use libc::{tcsetattr, STDIN_FILENO, TCSANOW};
use state::arena::Arena;
use state::food::Food;
use std::fs::File;
use std::io::Write;
use std::thread;
use std::time::Duration;

use state::directions::Directions;
use state::gamestate::GameState;

use engine::inputhandler::InputHandler;
use error::SnakeError;
use terminal::terminal::Terminal;

pub mod engine;
pub mod error;
pub mod random;
pub mod state;
pub mod terminal;

fn draw_snake(mut state: GameState) -> Result<GameState, SnakeError> {
    // If snake's head is at -1, -1 then this is a new game, so put snake in the middle
    if state.snake.positions[0].x == -1 || state.snake.positions[0].y == -1 {
        let (x_middle, y_middle) = Arena::middle_coords(&state.arena)?;

        state.snake.positions[0].x = x_middle as i16;
        state.snake.positions[0].y = y_middle as i16;

        return Ok(state);
    }

    // Clear the old snake
    for (i, p) in state.snake.positions.iter().enumerate() {
        // Clear all positions that don't have a facing or have and invalid position.
        // Always clear i when i is 0 as we want to make sure the starting piece is cleared.
        // Todo: add an is_valid() to Coords so we don't have to keep repeating this.
        if p.facing == Directions::None && (p.x == -1 || p.y == -1) && i > 0 {
            continue;
        }

        print!("\x1b[{};{}f", p.y, p.x);
        print!(" ");
    }

    state.snake.step();
    let mut snake_eaten = false;

    // Draw the new snake
    for (i, p) in state.snake.positions.iter().enumerate() {
        if p.x == -1 || p.y == -1 {
            continue;
        }

        // Did we eat something?
        if state.food.positions.iter().any(|&x| x == (p.x, p.y)) {
            state.score += 1;

            if !snake_eaten {
                snake_eaten = true;
            }
        }

        if state
            .arena
            .positions
            .iter()
            .map(|pos| (pos.0, pos.1))
            .any(|x| x == (p.x.try_into().unwrap(), p.y.try_into().unwrap()))
        {
            state.snake.x_x = true;
        }

        print!("\x1b[{};{}f", p.y, p.x);

        let previous_block_facing = match i {
            1.. => &state.snake.positions[i - 1].facing,
            _ => &p.facing,
        };

        // Draw the current block depending on the previous facing
        // vs. the current facing.  Draw corner pieces etc. accordingly.
        match (previous_block_facing, &p.facing) {
            (Directions::Down, Directions::Left) | (Directions::Right, Directions::Up) => {
                print!("╔");
            }
            (Directions::Up, Directions::Left) | (Directions::Right, Directions::Down) => {
                print!("╚");
            }
            (Directions::Down, Directions::Right) | (Directions::Left, Directions::Up) => {
                print!("╗")
            }
            (Directions::Left, Directions::Down) | (Directions::Up, Directions::Right) => {
                print!("╝")
            }
            (Directions::Down, Directions::Down) | (Directions::Up, Directions::Up) => {
                print!("║")
            }
            (Directions::Left, Directions::Left) | (Directions::Right, Directions::Right) => {
                print!("═")
            }
            _ => print!("║"),
        }
    }

    if snake_eaten {
        state.snake.grow(1);
        state = Food::new_random(state, 1)?;
    }

    match std::io::stdout().flush() {
        Ok(_) => Ok(state),
        Err(_) => Err(SnakeError),
    }

    // Todo: implement debug mode so we can see stuff like this in a bar at the bottom
    //println!("{:?}", &state);
}

fn draw_arena(state: &GameState) {
    for (x, y, char) in &state.arena.positions {
        print!("\x1b[{};{}f", y, x);
        print!("{}", char);
    }
}

fn draw_score(state: &GameState) {
    let (cols, _) = Terminal::get_console_size();

    print!("\x1b[{};{}f", 2, cols - 1);
    print!("{}", state.score);
}

fn game_loop(file: File) -> Result<(), SnakeError> {
    // Todo: move this out of game_loop and put into init() or main().
    let mut state = GameState::new();

    state = Arena::create_level_1(state);
    state = Food::new_random(state, 1)?;

    loop {
        state = InputHandler::handle_input(state, &file);
        state = draw_snake(state)?;
        draw_arena(&state);
        draw_score(&state);

        if state.snake.x_x {
            break;
        }

        //thread::sleep(Duration::from_millis(16)); // about 60 fps
        thread::sleep(Duration::from_millis(200));
    }

    Ok(())
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

    // Todo: get the current console size and store it in the game state.

    if game_loop(file).is_ok() {
        // todo: clear screen function here.
        print!("\x1b[H");
        print!("\x1b[2J");
        println!("x_x you died.");
    }

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}
