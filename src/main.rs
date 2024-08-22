extern crate libc;

use libc::{termios, TCSANOW, tcgetattr, tcsetattr, STDIN_FILENO, ioctl, winsize, TIOCGWINSZ};
use std::io::Read;
use std::mem;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;

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

fn set_non_blocking_stdin() -> File {
    let stdin = 0;
    let file = unsafe { File::from_raw_fd(stdin) };
    let fd = file.as_raw_fd();

    unsafe {
        let flags = libc::fcntl(fd, libc::F_GETFL, 0);
        libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
    }

    file
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

fn handle_input(mut state: GameState, mut file: &File) -> GameState {
    if state.snake.positions[0].x == 0 || state.snake.positions[0].y == 0 {
        return state
    }

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
    // If snake's head is at 0,0 then this is a new game, so put snake in the middle
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

    state.snake.step();

    match state.snake.direction {
        Directions::Down => println!("║"),
        Directions::Up =>  println!("║"),
        Directions::Left => println!("═"),
        Directions::Right => println!("═"),
        _ => println!("║")
    };

    // Todo: implement debug mode so we can see stuff like this in a bar at the bottom
    //println!("{:?}", &state);

    state
}

fn game_loop(file: File) {
    let mut state = GameState {
        snake: Snake {
            positions: [Coords { x: 0, y: 0 }; 20],
            direction: Directions::Up
        },
    };

    loop {
        state = handle_input(state, &file);
        state = draw_snake(state);

        //thread::sleep(Duration::from_millis(16)); // about 60 fps
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    let original_term = set_raw_mode();
    let file = set_non_blocking_stdin();

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

// Move these
#[derive(Debug, Copy, Clone)]
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

impl PartialEq for Directions {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Debug)]
pub struct Snake {
    // The position of each block making up the body of snake
    // and [0] being the head
    // The idea is that when snake is moving e.g. left, block 0
    // Y would be decreasing on each tick.
    // This array should be looped through on each tick so that 
    // we can update all part of snakes body according to the current direction.
    // Snake would want to look like he's moving in that direction, and so
    // on each tick we would need to remove the last element, add a new element
    // to the top of the array which would be in the position the head has moved to
    positions: [Coords; 20],

    // Holds the direction snake's head is currently facing
    direction: Directions,
}

impl Snake {
    pub fn step(&mut self) -> &mut Snake {
        // if snake is 1, just move forward - erase old block, write new block
        // if he is > 1, pop bit off tail and push on top in the direction you're going

        match self.direction {
            Directions::Up => self.positions[0].y -= 1,
            Directions::Down => self.positions[0].y += 1,
            Directions::Left => self.positions[0].x -= 1,
            Directions::Right => self.positions[0].x += 1,
            _ => {}
        }

        self
    }
}

#[derive(Debug)]
pub struct GameState {
    snake: Snake
}