extern crate libc;

use libc::{termios, TCSANOW, tcgetattr, tcsetattr, STDIN_FILENO};
use std::io::{self, Read};
use std::mem;

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

fn handle_input() {
    let mut buffer = [0; 3]; // Buffer to store input characters

    while let Ok(_) = io::stdin().read(&mut buffer) {
        if buffer[0] == 0x18 && buffer[1] == 0x00 && buffer[2] == 0x00 {
            println!("CTRL+X");
        }

        if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x44 {
            println!("Left Arrow");
        }

        if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x43 {
            println!("Right Arrow");
        }

        if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x41 {
            println!("Up Arrow");
        }

        if buffer[0] == 0x1B && buffer[1] == 0x5B && buffer[2] == 0x42 {
            println!("Down Arrow");
        }
        
        buffer = [0; 3]; // Clear buffer for next input
    }
}

fn main() {
    // Set terminal to raw mode
    let original_term = set_raw_mode();

    // Clear the screen
    print!("\x1b[2J");

    // Move to 0
    print!("\x1b[H");

    println!("Welcome to terminal_snake");

    // Handle key presses
    handle_input();

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}
