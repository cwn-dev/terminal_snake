extern crate libc;

use libc::{ioctl, termios, winsize, TIOCGWINSZ, TCSANOW, tcgetattr, tcsetattr, STDIN_FILENO};
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

fn handle_key_presses() {
    let mut buffer = [0; 1]; // Buffer to store input characters

    while let Ok(_) = io::stdin().read_exact(&mut buffer) {
        if buffer[0] == 0x18 {
            println!("CTRL+X");
        }
        
        buffer = [0; 1]; // Clear buffer for next input
    }
}

fn print_line() {
    print!("\x1b[2;1H");

    for n in 3..5 {
        println!("║");
        print!("\x1b[{};1H", n);
    }
}

fn main() {
    // Set terminal to raw mode
    let original_term = set_raw_mode();

    // Clear the screen
    print!("\x1b[2J");

    // Move to 0
    print!("\x1b[H");

    // Get console size
    let (width, height) = get_console_size();
    println!("Console Width: {}, Console Height: {}", width, height);

    print_line();

    // Handle key presses
    handle_key_presses();

    // Restore original terminal settings
    unsafe {
        tcsetattr(STDIN_FILENO, TCSANOW, &original_term);
    }
}
