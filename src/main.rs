extern crate libc;

use libc::{termios, TCSANOW, tcgetattr, tcsetattr, STDIN_FILENO, ioctl, winsize, TIOCGWINSZ};
use std::io::{self, Read};
use std::mem;
use std::thread;
use std::time::Duration;

fn set_raw_mode() -> termios {
    let mut term = unsafe { mem::zeroed::<termios>() };

    // Get terminal attributes
    unsafe {
        tcgetattr(STDIN_FILENO, &mut term);
    }

    // Save original attributes for later restoration
    let original_term = term;

    // Set terminal to raw mode
    term.c_lflag &= !(libc::ICANON | libc::ECHO); // Disable canonical mode and echo
    term.c_cc[libc::VMIN] = 1; // Minimum number of characters to read
    term.c_cc[libc::VTIME] = 0; // Timeout in deciseconds

    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &term);
    }

    original_term
}

fn get_console_size() -> (u16, u16) {
    let mut ws = winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    unsafe {
        ioctl(STDIN_FILENO, TIOCGWINSZ, &mut ws);
    }

    (ws.ws_col as u16, ws.ws_row as u16)
}

fn handle_input(mut state: GameState) -> GameState {
    if state.snake.positions[0].x == 0 || state.snake.positions[0].y == 0 {
        return state
    }

    let mut buffer = [0; 3]; // Buffer to store input characters

    let _ = io::stdin().read(&mut buffer);
    if buffer[0] == 0x18 && buffer[1] == 0x00 && buffer[2] == 0x00 {
        //println!("CTRL+X");
    }

    // Left Arrow
    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x44 {
        state.snake.positions[0].x -= 1;
        state.snake.direction = Directions::Left;
    }

    // Right Arrow
    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x43 {
        state.snake.positions[0].x += 1;
        state.snake.direction = Directions::Right;
    }

    // Up Arrow
    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x41 {        
        state.snake.positions[0].y -= 1;
        state.snake.direction = Directions::Up;
    }

    // Down Arrow
    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x42 {
        state.snake.positions[0].y += 1;
        state.snake.direction = Directions::Down;
    }

    buffer = [0; 3]; // Clear buffer for next input

    state
}

fn draw_snake(mut state: GameState) -> GameState {
    if state.snake.positions[0].x == 0 || state.snake.positions[0].y == 0 {
        let (cols, rows) = get_console_size();
        let middle_x = (cols + 1) / 2;
        let middle_y = (rows + 1) / 2;

        state.snake.positions[0].x = middle_x;
        state.snake.positions[0].y = middle_y;

        return state
    }

    let snake_head_pos = &state.snake.positions[0];
    print!("\x1b[{};{}f", snake_head_pos.y, snake_head_pos.x);

    match state.snake.direction {
        Directions::Down => println!("║"),
        Directions::Up =>  println!("║"),
        Directions::Left => println!("═"), // 2x ═ ═ - may have to double array size when moving horizontally
        Directions::Right => println!("═"), // 2x ═ ═  ^^
        _ => println!("║")
    };

    // Todo: implement debug mode so we can see stuff like this in a bar at the bottom
    //println!("{:?}", &state);

    state
}

fn game_loop() {
    let mut state = GameState {
        snake: SnakeStatus {
            positions: [Coords { x: 0, y: 0 }; 1],
            direction: Directions::None
        }
    };

    loop {
        state = handle_input(state);
        state = draw_snake(state);

        thread::sleep(Duration::from_secs(1/60));
    }
}

fn main() {
    // Set terminal to raw mode
    let original_term = set_raw_mode();

    // Clear the screen
    print!("\x1b[2J");

    // Move to 0
    print!("\x1b[H");

    // Hide the cursor
    print!("\x1b[?25l");

    println!("Welcome to terminal_snake");
    println!("Press any key to start");
    
    game_loop();

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}

// Move these
#[derive(Debug)]
pub struct Coords {
    x: u16,
    y: u16
}

#[derive(Debug)]
enum Directions {
    None,
    Up,
    Down,
    Right,
    Left
}

#[derive(Debug)]
pub struct SnakeStatus {
    // The position of each block making up the body of snake
    // and [0] being the head
    // The idea is that 
    positions: [Coords; 1],

    // Holds the direction snake's head is currently facing
    direction: Directions,
}

#[derive(Debug)]
pub struct GameState {
    snake: SnakeStatus
}