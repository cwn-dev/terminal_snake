extern crate libc;

use libc::{tcsetattr, STDIN_FILENO, TCSANOW};
use random::random::Random;
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
        let (cols, rows) = Terminal::get_console_size();
        let middle_x = (cols + 1) / 2;
        let middle_y = (rows + 1) / 2;

        state.snake.positions[0].x = middle_x as i16;
        state.snake.positions[0].y = middle_y as i16;

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
    for p in state.snake.positions.iter() {
        if p.x == -1 || p.y == -1 {
            continue;
        }

        // Did we eat something?
        match state.food.positions.iter().find(|&&x| x == (p.x, p.y)) {
            Some(_) => {
                state.score += 1;

                if !snake_eaten {
                    snake_eaten = true;
                }
            }
            None => (),
        }

        print!("\x1b[{};{}f", p.y, p.x);

        match p.facing {
            Directions::Down => print!("║"),
            Directions::Up => print!("║"),
            Directions::Left => print!("═"),
            Directions::Right => print!("═"),
            _ => print!("║"),
        }
    }

    if snake_eaten {
        state.snake.grow(1);
        state = drawn_random_food(state);
    }

    match std::io::stdout().flush() {
        Ok(_) => {
            return Ok(state);
        }
        Err(_) => Err(SnakeError),
    }

    // Todo: implement debug mode so we can see stuff like this in a bar at the bottom
    //println!("{:?}", &state);
}

fn draw_arena() {
    let (cols, rows) = Terminal::get_console_size();

    // Todo: we need some way of tracking where other blocks have been drawn on
    // so we don't have to manually track stuff like this starting on row 3...

    // Draw the corners
    print!("\x1b[{};{}f", 3, cols - 1); // top right
    print!("╮");
    print!("\x1b[{};{}f", rows - 1, cols - 1); // bottom right
    print!("╯");
    print!("\x1b[{};{}f", rows - 1, 1); // bottom left
    print!("╰");
    print!("\x1b[{};{}f", 3, 1); // top left
    print!("╭");

    // Draw the top line
    for i in 2..cols - 1 {
        print!("\x1b[{};{}f", 3, i);
        print!("─");
    }

    // Draw the right line
    for i in 4..rows - 1 {
        print!("\x1b[{};{}f", i, cols - 1);
        print!("│");
    }

    // Draw the bottom line
    for i in 2..cols - 1 {
        print!("\x1b[{};{}f", rows - 1, i);
        print!("─");
    }

    // Draw the left line
    for i in 4..rows - 1 {
        print!("\x1b[{};{}f", i, 1);
        print!("│");
    }
}

fn drawn_random_food(mut state: GameState) -> GameState {
    let (cols, rows) = Terminal::get_console_size();
    let rand_cols = Random::time_seed().get(2, (cols - 3) as u128);
    let rand_rows = Random::time_seed().get(4, (rows - 3) as u128);

    state.food.positions[0] = (rand_cols as i16, rand_rows as i16);

    print!("\x1b[{};{}f", rand_rows, rand_cols);
    print!("▫");

    state
}

fn draw_score(state: &GameState) {
    let (cols, _) = Terminal::get_console_size();

    print!("\x1b[{};{}f", 2, cols - 1);
    print!("{}", state.score);
}

fn game_loop(file: File) -> Result<(), SnakeError> {
    // Todo: move this out of game_loop and put into init() or main().
    let mut state = GameState::new();

    state = drawn_random_food(state);

    loop {
        state = InputHandler::handle_input(state, &file);
        state = draw_snake(state)?;
        draw_score(&state);

        //thread::sleep(Duration::from_millis(16)); // about 60 fps
        thread::sleep(Duration::from_millis(200));
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

    // Todo: get the current console size and store it in the game state.

    draw_arena();
    match game_loop(file) {
        Ok(_) => println!("Test"),
        Err(_) => {}
    }

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}
