use std::io::Read;
use libc::{STDIN_FILENO, STDOUT_FILENO, TIOCGWINSZ, VMIN, VTIME, iscntrl};
use nix::{pty::Winsize, sys::termios, sys::termios::LocalFlags};
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
    let flags = [LocalFlags::ECHO, LocalFlags::ICANON, LocalFlags::ISIG];
    for flag in flags.iter(){
        term.local_flags.remove(*flag);
    }
    term.input_flags.remove(termios::InputFlags::IXON);
    set_termios(&term);
    Ok(orig_termios)
}

fn disable_raw_mode(term:&termios::Termios) -> Result<(), Error>{
   set_termios(term);
    Ok(())
}

fn main() {
    let old_term = match enable_raw_mode() {
        Ok(term) => term,
        Err(error) => panic!("Error when enable raw mode: {}", error),
    };
    //println!("{:?}", std::io::stdin().bytes().next());
    for ch in std::io::stdin().bytes() {
        let ch = match ch {
            Ok(ch) => ch as char,
            Err(error) => panic!("Error when read from stdin: {}", error),
        };

        if ch == 'q' {
            break;
        }
        let is_cntrl;
        unsafe {
            is_cntrl = iscntrl(ch as i32) == 0;
        }
        if is_cntrl{
            println!("{}", ch);
        }
        else{
            println!("{:x}", ch as u8);
        }
    }
    disable_raw_mode(&old_term);
}
