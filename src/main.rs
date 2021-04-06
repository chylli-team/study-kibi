use std::io::Read;
use libc::{STDIN_FILENO, STDOUT_FILENO, TIOCGWINSZ, VMIN, VTIME, ECHO};
use nix::{pty::Winsize, sys::termios};
use study_kibi::{ansi_escape::*, Error};

fn set_termios(term:&termios::Termios) -> Result<(), nix::Error>{
    termios::tcsetattr(STDIN_FILENO, termios::SetArg::TCSAFLUSH, term)
}

/// Setup the termios to enable raw mode, and return the original termios.
///
/// termios manual is available at: <http://man7.org/linux/man-pages/man3/termios.3.html>
fn enable_raw_mode() -> Result<termios::Termios, Error> {
    let orig_termios = termios::tcgetattr(STDIN_FILENO)?;
    let mut term = orig_termios.clone();
    //termios::cfmakeraw(&mut term);
    // Set the minimum number of characters for non-canonical reads
    //term.control_chars[VMIN] = 0;
    // Set the timeout in deciseconds for non-canonical reads
    //term.control_chars[VTIME] = 1;
    //term.local_flags.insert(termios::LocalFlags::ECHO);
    term.local_flags.remove(termios::LocalFlags::ECHO);
    set_termios(&term)?;
    Ok(orig_termios)
}
fn main() {
    enable_raw_mode();
    println!("{:?}", std::io::stdin().bytes().next());
    //while 'q' as u8 != std::io::stdin().bytes().next()? {

//    }
}
