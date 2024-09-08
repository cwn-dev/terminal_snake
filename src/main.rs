extern crate libc;

use engine::graphics::Graphics;
use engine::snengine_error::SnengineError;
use engine::unicode::Unicode;
use libc::{tcsetattr, termios, STDIN_FILENO, TCSANOW};
use state::arena::Arena;
use state::food::Food;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::{Duration, Instant};
use std::{i16, thread};

use state::directions::Directions;
use state::gamestate::GameState;

use engine::inputhandler::InputHandler;
use terminal::terminal::Terminal;

pub mod engine;
pub mod error;
pub mod random;
pub mod state;
pub mod terminal;

fn draw_snake(mut state: GameState) -> Result<GameState, Box<dyn Error>> {
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

        let (ux, uy) = p.to_unsigned_tuple();

        Graphics::draw_char(ux, uy, Unicode::Space)?;
    }

    state.snake.step();
    let mut snake_eaten = false;

    // Draw the new snake
    for (i, p) in state.snake.positions.iter().enumerate() {
        // Continue if coordinates are off screen
        let (x, y): (u16, u16) = match (p.x, p.y) {
            (_, _) if p.x == -1 || p.y == -1 => continue,
            (_, _) => (p.x.try_into().unwrap(), p.y.try_into().unwrap()), // YOLO
        };

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

        let previous_block_facing = match i {
            1.. => &state.snake.positions[i - 1].facing,
            _ => &p.facing,
        };

        // Draw the current block depending on the previous facing
        // vs. the current facing.  Draw corner pieces etc. accordingly.
        match (previous_block_facing, &p.facing) {
            (Directions::Down, Directions::Left) | (Directions::Right, Directions::Up) => {
                Graphics::draw_char(x, y, Unicode::BoxDoubleDownAndRight)?
            }
            (Directions::Up, Directions::Left) | (Directions::Right, Directions::Down) => {
                Graphics::draw_char(x, y, Unicode::BoxDoubleUpAndRight)?
            }
            (Directions::Down, Directions::Right) | (Directions::Left, Directions::Up) => {
                Graphics::draw_char(x, y, Unicode::BoxDoubleDownAndLeft)?
            }
            (Directions::Left, Directions::Down) | (Directions::Up, Directions::Right) => {
                Graphics::draw_char(x, y, Unicode::BoxDoubleUpAndLeft)?
            }
            (Directions::Left, Directions::Left) | (Directions::Right, Directions::Right) => {
                Graphics::draw_char(x, y, Unicode::BoxDoubleHorizontal)?
            }
            _ => {
                // Down, Up and None
                Graphics::draw_char(x, y, Unicode::BoxDoubleVertical)?
            }
        }
    }

    if snake_eaten {
        state.snake.grow(1);
        state = Food::new_random(state, 1)?;
        draw_food(&state)?;
    }

    match std::io::stdout().flush() {
        Ok(_) => Ok(state),
        Err(e) => Err(e.into()),
    }

    // Todo: implement debug mode so we can see stuff like this in a bar at the bottom
    //println!("{:?}", &state);
}

fn draw_arena(state: &GameState) -> Result<(), SnengineError> {
    // todo: add DrawingError?
    for (x, y, char) in &state.arena.positions {
        Graphics::draw_char(*x, *y, char.clone())?;
    }

    Ok(())
}

fn draw_score(state: &GameState) -> Result<(), SnengineError> {
    let (cols, _) = Terminal::get_console_size();

    Graphics::write(cols - 1, 2, state.score.to_string())
}

fn draw_food(state: &GameState) -> Result<(), SnengineError> {
    for (x, y) in state.food.positions {
        if x == -1 || y == -1 {
            continue;
        }

        // Todo: make food.positions use the Coords struct
        // so that we don't have to do this silliness...
        let x_u: u16 = x.try_into().unwrap();
        let y_u: u16 = y.try_into().unwrap();

        Graphics::draw_char(x_u, y_u, Unicode::HeavyCircleWithCircleInside)?
    }

    Ok(())
}

fn game_loop(file: File) -> Result<(), Box<dyn Error>> {
    // Todo: move this out of game_loop and put into init() or main().
    let mut state = GameState::new();

    state = Arena::create_level_1(state);
    state = Food::new_random(state, 1)?;
    state = draw_snake(state)?;
    draw_food(&state)?;

    let mut time_since_draw = Instant::now();

    loop {
        state = InputHandler::handle_input(state, &file);
        draw_arena(&state)?;
        draw_score(&state)?;

        if state.snake.x_x {
            break;
        }

        // Update Snake only after the given duration has passed.
        // This means Snake remains controllable while having fast updates and input.
        if time_since_draw.elapsed() >= Duration::from_millis(100) {
            state = draw_snake(state)?;
            time_since_draw = Instant::now();
        }

        thread::sleep(Duration::from_millis(8)); // about 120 fps
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

    // <Esc>[38;2;<FG_R>;<FG_G>;<FG_B>;<BG_R>;<BG_G>;<BG_B>m
    println!("\x1b[38;2;0;72;186;48;2;255;255;255mWelcome to terminal_snake\x1b[0m");
    println!("Press any key to start");

    // Todo: get the current console size and store it in the game state.

    match game_loop(file) {
        Ok(_) => {
            // todo: clear screen function here.
            print!("\x1b[H");
            print!("\x1b[2J");
            println!("x_x you died.");
        }
        Err(e) => {
            // todo: clear screen function here.
            print!("\x1b[H");
            print!("\x1b[2J");

            println!("{}", e);
        }
    }

    restore_terminal(&original_term)
}

fn restore_terminal(terminal: &termios) {
    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, terminal);
    }
}
