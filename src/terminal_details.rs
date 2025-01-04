use std::ffi::c_ushort;
use std::time::Duration;

use nix::libc::{ioctl, STDOUT_FILENO};
use timeout_readwrite::TimeoutReader;

use std::io::{stdin, stdout, Read};
use std::io::{IsTerminal, Write};
use std::os::fd::AsRawFd;
use termios::*;

#[repr(C)]
pub struct WinSize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

pub fn get_window_size() -> (u16, u16) {
    let winsize = WinSize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };
    /* this is meant to be the fd to use, but for some reason it doesn't work - STDOUT_FILENO is what works in this mac */
    /* let fd = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .map(|file| file.as_raw_fd())
            .unwrap();
    */
    unsafe {
        let retval = ioctl(STDOUT_FILENO, nix::libc::TIOCGWINSZ, &winsize);

        if retval != 0 {
            panic!("ioctl failed - returned {}", retval);
        }
    }
    //ioctl(io::stdout().as_raw_fd(), TIOCGWINSZ, &mut winsize).unwrap();
    (winsize.ws_col, winsize.ws_row)
}

/* checks if the terminal supports graphics
   https://sw.kovidgoyal.net/kitty/graphics-protocol/#a-minimal-example

   we timeout after 10ms, which is a bit of a long time, but it's a good long time.
*/
pub fn get_kitty_support() -> bool {
    run_code_in_raw_mode(|| {
        let reader = stdin();

        let mut reader_with_timeout =
            TimeoutReader::new(reader.lock(), Duration::new(0, 10_000_000)); // 10ms
        let return_val = "\u{1b}_Gi=4294967295;OK\u{1b}\\";

        let mut buffer = [0; 20]; // read exactly 20 bytes - the string above

        print!("\u{1b}_Gi=4294967295,s=1,v=1,a=q,t=d,f=24;AAAA\u{1b}\\");

        stdout().flush().unwrap(); // this is critical to make the terminal get the above

        let res = reader_with_timeout.read_exact(&mut buffer);

        match res {
            Ok(_) => {
                //println!(" {:?} {}", buffer, buffer.escape_ascii());
                if buffer == return_val.as_bytes() {
                    return true;
                }
            }
            Err(_e) => {
                // we know the timeout occured, so ignore it.
            }
        }
        //println!(" {:?} {}", buffer, buffer.escape_ascii());

        false
    })
}

/**
 * uses an ANSI escape sequence to get the terminal dimensions in pixels, supported by most modern terminals
 */
pub fn get_terminal_dimensions_in_pixels() -> Result<(u16, u16), String> {
    if !stdout().is_terminal() {
        return Err("This function needs a terminal".to_string());
    }

    run_code_in_raw_mode(|| {
        let mut reader = stdin().lock();
        let mut buffer = [0; 4]; // read exactly one byte
        print!("\u{1b}[14t");
        stdout().flush().unwrap(); // this is critical to make the terminal get the above

        reader.read_exact(&mut buffer).unwrap();
        //println!("You have hit: {:?} {}", buffer, buffer.escape_ascii());

        let mut dimensions = (0, 0);

        // if we got back \x1b[4; then it's the terminal and we need to read until 't'
        if buffer == [27, 91, 52, 59] {
            //println!("Your terminal responded to the escape sequence.");

            let mut charbuffer = [0; 1];
            let mut success = false;

            let mut outstr = String::new();
            // 20 is a reasonable limit if we can't get our answer by then, normally it takes me 10 characters
            for _ in 0..20 {
                reader.read_exact(&mut charbuffer).unwrap();
                let char = charbuffer[0];

                if char == b't' {
                    success = true;
                    break;
                }
                outstr.push(char as char);
            }

            if success {
                //println!("fetched eek. {}", outstr);

                let outstr: Vec<&str> = outstr.split(';').collect();
                //println!(" {:?}", outstr);
                dimensions = (
                    outstr[1].parse::<u16>().unwrap(),
                    outstr[0].parse::<u16>().unwrap(),
                );
            }
        } else {
            return Err("Your terminal did not respond to the escape sequence \\e[14t".to_string());
        }
        Ok(dimensions)
    })
}

/**
 * allows you to go to raw mode, run a function, and then return to normal mode and return the result
 * useful for when you want to ask the terminal something
 * */
pub fn run_code_in_raw_mode<T>(func: fn() -> T) -> T {
    let stdin_fd = stdin().as_raw_fd();
    let mut termios = Termios::from_fd(stdin_fd).unwrap();
    let old_termios = termios.clone();

    termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin_fd, TCSANOW, &mut termios).unwrap();
    cfmakeraw(&mut termios);

    let return_value = func();

    tcsetattr(stdin_fd, TCSANOW, &old_termios).unwrap(); // reset the stdin to the original termios data
    return_value
}
