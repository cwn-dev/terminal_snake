extern crate libc;

use engine::coords::Coords;
use engine::graphics::Graphics;
use engine::snengine_error::SnengineError;
use engine::unicode::Unicode;
use libc::{tcsetattr, termios, STDIN_FILENO, TCSANOW};
use state::arena::Arena;
use state::food::Food;
use state::snake::Snake;
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
    /*
     * If the direction of the snake is something other than None, it should be
     * moving, so mark the head block as active!
     */
    if state.snake.direction != Directions::None {
        state.snake.positions[0].active = true;
    }

    // If snake's head is not active then this is a new game, so put snake in the middle
    if !state.snake.positions[0].active {
        let (x_middle, y_middle) = Arena::middle_coords(&state.arena)?;

        state.snake.positions[0].coords.x = x_middle as i16;
        state.snake.positions[0].coords.y = y_middle as i16;

        return Ok(state);
    } else {
        state.snake.clear()?;
        state.snake.step();
    }

    let mut snake_eaten = false;

    // Draw the new snake
    for (i, p) in state.snake.positions.iter().enumerate() {
        let (x, y) = p.coords.to_unsigned_tuple();

        if x == 0 || y == 0 {
            continue;
        }

        if Snake::has_hit_self(&state.snake) {
            state.snake.x_x = true;
        }

        // Did we eat something?
        if state
            .food
            .positions
            .iter()
            .any(|&x| x == Coords::new(p.coords.x, p.coords.y))
        {
            state.score += 1;

            if !snake_eaten {
                snake_eaten = true;
            }
        }

        // Check if Snake hit an arena wall piece
        if state
            .arena
            .positions
            .iter()
            .any(|c| c.0 == Coords::new(p.coords.x, p.coords.y))
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
}

fn draw_arena(state: &GameState) -> Result<(), SnengineError> {
    // todo: add DrawingError?
    for (coords, char) in &state.arena.positions {
        let (x, y) = coords.to_unsigned_tuple();
        Graphics::draw_char(x, y, char.clone())?;
    }

    Ok(())
}

fn draw_score(state: &GameState) -> Result<(), SnengineError> {
    let (cols, _) = state.c_dimensions.to_unsigned_tuple();

    Graphics::write(cols - 1, 2, &state.score.to_string())
}

fn draw_food(state: &GameState) -> Result<(), SnengineError> {
    for c in state.food.positions {
        if !c.is_active() {
            continue;
        }

        let (x, y) = c.to_unsigned_tuple();

        Graphics::draw_char(x, y, Unicode::HeavyCircleWithCircleInside)?
    }

    Ok(())
}

fn draw_diags(state: &GameState) -> Result<(), SnengineError> {
    // We'll draw the diagnostic output on the bottom row.
    let (c_x, c_y) = state.c_dimensions.to_unsigned_tuple();

    for i in 1..c_x {
        Graphics::draw_char(i, c_y, Unicode::Space)?;
    }

    // Displays `Facing: Up`
    Graphics::write(
        1,
        c_y,
        format!("Current facing: {:?}", state.snake.direction).as_str(),
    )?;

    // Displays head coordinates
    // Graphics::write(
    //     30,
    //     c_y,
    //     format!("Head coords: {:?}", state.snake.positions[0]).as_str()
    // )?;

    for i in 1..c_x {
        Graphics::draw_char(i, c_y - 1, Unicode::BoxLightHorizontal)?;
    }

    Ok(())
}

fn game_loop(file: File) -> Result<GameState, Box<dyn Error>> {
    // Todo: move this out of game_loop and put into init() or main().
    let mut state = GameState::new();

    // Save the current console size to GameState for easy access.
    let (c_x, c_y) = Terminal::get_console_size();
    state.c_dimensions = Coords::new(c_x as i16, c_y as i16);

    state = Arena::create_level_1(state);
    state = Food::new_random(state, 1)?;
    state = draw_snake(state)?;
    draw_food(&state)?;

    let mut time_since_draw = Instant::now();

    loop {
        draw_arena(&state)?;
        draw_score(&state)?;
        draw_diags(&state)?;

        if state.snake.x_x {
            break;
        }

        // Update Snake only after the given duration has passed.
        // This means Snake remains controllable while having fast updates and input.
        if time_since_draw.elapsed() >= Duration::from_millis(100) {
            state = InputHandler::handle_input(state, &file);
            state = draw_snake(state)?;
            time_since_draw = Instant::now();
        }

        thread::sleep(Duration::from_millis(8)); // about 120 fps
    }

    Ok(state)
}

fn main() {
    let original_term = Terminal::set_raw_mode();
    let file = Terminal::set_non_blocking_stdin();

    Graphics::clear_screen();

    // Hide the cursor
    print!("\x1b[?25l");

    // <Esc>[38;2;<FG_R>;<FG_G>;<FG_B>;<BG_R>;<BG_G>;<BG_B>m
    println!("\x1b[38;2;0;72;186;48;2;255;255;255mWelcome to terminal_snake\x1b[0m");
    println!("Press any key to start");

    // Todo: we have an inefficiency here where the game will sit idle
    // on the 'welcome' screen taking CPU.  This is because the loop starts
    // regardless of whether or not the user has pressed a button. Ultimately
    // we want to show a start button and the result of that function would
    // start the game loop. To be honest, it might be that we want the loop
    // to start regardless if we have a good looking start screen, it might
    // have animation or at minimum a Start, Options and Exit buttons which
    // would highlight on select, meaning there would need to be some kind of
    // loop.

    match game_loop(file) {
        Ok(g) => {
            //Graphics::clear_screen();
            println!("x_x you died. You got {}...", g.score);
        }
        Err(e) => {
            Graphics::clear_screen();

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
