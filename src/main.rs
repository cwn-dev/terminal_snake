extern crate libc;

use libc::{ TCSANOW, tcsetattr, STDIN_FILENO };
use random::random::Random;
use state::food::Food;
use std::io::Write;
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
pub mod random;

fn draw_snake(mut state: GameState) -> GameState {
    // If snake's head is at -1, -1 then this is a new game, so put snake in the middle
    if state.snake.positions[0].x == -1 || state.snake.positions[0].y == -1 {
        let (cols, rows) = Terminal::get_console_size();
        let middle_x = (cols + 1) / 2;
        let middle_y = (rows + 1) / 2;

        state.snake.positions[0].x = middle_x as i16;
        state.snake.positions[0].y = middle_y as i16;

        return state;
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

    // Draw the new snake
    for p in state.snake.positions.iter() {
        if p.x == -1 || p.y == -1 {
            continue;
        }

        // Did we eat something?
        match state.food.positions.iter().find(|&&x| x == (p.x, p.y)) {
            Some(_) => {
                state.score += 1;
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

        match std::io::stdout().flush() {
            Ok(_) => {
                return state;
            }
            Err(_) => panic!("Umm, stdout() failed I think 🤷‍♂️"),
        }
    }

    // Todo: implement debug mode so we can see stuff like this in a bar at the bottom
    //println!("{:?}", &state);

    state
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

fn game_loop(file: File) {
    // Todo: move this out of game_loop and put into init() or main().
    let mut state = GameState {
        score: 0,
        snake: Snake {
            positions: [Coords { x: -1, y: -1, facing: Directions::None }; 20],
            direction: Directions::None,
        },
        food: Food {
            positions: [(-1, -1); 3],
        },
    };

    state = drawn_random_food(state);

    loop {
        state = InputHandler::handle_input(state, &file);
        state = draw_snake(state);

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
    game_loop(file);

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}
