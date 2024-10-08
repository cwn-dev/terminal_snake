use libc::{ioctl, tcgetattr, tcsetattr, termios, winsize, STDIN_FILENO, TCSANOW, TIOCGWINSZ};
use std::fs::File;
use std::mem;
use std::os::unix::io::AsRawFd;
use std::os::unix::io::FromRawFd;

pub struct Terminal {}

impl Terminal {
    pub fn set_raw_mode() -> termios {
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

    pub fn set_non_blocking_stdin() -> File {
        let stdin = 0;
        let file = unsafe { File::from_raw_fd(stdin) };
        let fd = file.as_raw_fd();

        unsafe {
            let flags = libc::fcntl(fd, libc::F_GETFL, 0);
            libc::fcntl(fd, libc::F_SETFL, flags | libc::O_NONBLOCK);
        }

        file
    }

    pub fn get_console_size() -> (u16, u16) {
        let mut ws = winsize {
            ws_row: 0,
            ws_col: 0,
            ws_xpixel: 0,
            ws_ypixel: 0,
        };

        unsafe {
            ioctl(STDIN_FILENO, TIOCGWINSZ, &mut ws);
        }

        // Error check this conversion.
        (ws.ws_col, ws.ws_row)
    }
}
