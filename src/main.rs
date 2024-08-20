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

fn handle_input() {
    let mut buffer = [0; 3]; // Buffer to store input characters

    let _ = io::stdin().read(&mut buffer);
    if buffer[0] == 0x18 && buffer[1] == 0x00 && buffer[2] == 0x00 {
        //println!("CTRL+X");
    }

    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x44 {
        //println!("Left Arrow");
    }

    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x43 {
        //println!("Right Arrow");
    }

    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x41 {
        //println!("Up Arrow");
    }

    if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x42 {
        //println!("Down Arrow");
    }

    buffer = [0; 3]; // Clear buffer for next input
}

pub struct Coords {
    x: u16,
    y: u16
}

enum Directions {
    Up,
    Down,
    Right,
    Left
}

pub struct SnakeStatus {
    position: Coords,
    direction: Directions
}

fn draw_snake() {
    let (cols, rows) = get_console_size();
    let middle_x = (cols + 1) / 2;
    let middle_y = (rows + 1) / 2;

    print!("\x1b[{};{}f", middle_y, middle_x);
    println!("║");
}

fn game_loop() {
    loop {
        handle_input();
        draw_snake();

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
    
    game_loop();

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}
