use std::ffi::c_ushort;
use std::fs::OpenOptions;

use nix::libc::{ioctl, STDOUT_FILENO};

use std::io::Read;
use std::io::Write;
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
        let mut tty_device = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open("/dev/tty")
            .unwrap();

        let return_val = "Gi=4294967295;OK\u{1b}\\";

        let _res = write!(
            tty_device,
            "\u{1b}_Gi=4294967295,s=1,v=1,a=q,t=d,f=24;AAAA\u{1b}\\\u{1b}[c"
        );

        let out = read_ansi_stream(&mut tty_device);
        // println!("deadbeef {} {:?}", out, out);
        // println!("deadbeef {} {:?}", return_val, return_val);
        if out == return_val {
            let _out = read_ansi_stream(&mut tty_device);
            // println!("deadbeef {} {:?}", out, out);
            // println!("deadbeef {} {:?}", return_val, return_val);
            return true;
        }

        false
    })
}

pub fn read_ansi_stream(reader: &mut dyn Read) -> String {
    let a = read_one_character(reader).unwrap();
    loop {
        if a == '\u{1b}' {
            let b = read_one_character(reader).unwrap();

            if b == '[' {
                return read_until(reader, "c");
            } else if b == '_' {
                return read_until(reader, "\\");
            } else {
                // something went wrong
                println!("unknown ansi sequence \\e{}", b);
            }
        } else {
            // something went wrong, non escape character, do nothing.
            println!("got {:?}", a);
        }
    }
}

pub fn read_one_character(reader: &mut dyn Read) -> Result<char, String> {
    let mut buffer = [0; 1]; // read exactly one byte

    let res = reader.read_exact(&mut buffer);

    match res {
        Ok(_) => {
            let char = buffer[0] as char;
            Ok(char)
        }
        Err(e) => Err(format!("error reading character: {:?}", e)),
    }
}

pub fn read_until(reader: &mut dyn Read, escape_terminator: &str) -> String {
    let mut buffer = String::new();
    let mut _complete = false;

    for _c in 0..30 {
        let char = read_one_character(reader).unwrap();
        buffer.push(char);
        if buffer.contains(escape_terminator) {
            _complete = true;
            break;
        }
    }

    buffer
}

/**
 * uses an ANSI escape sequence to get the terminal dimensions in pixels, supported by most modern terminals
 */
pub fn get_terminal_dimensions_in_pixels() -> Result<(u16, u16), String> {
    // if !stdout().is_terminal() {
    //     return Err("This function needs a terminal".to_string());
    // }

    run_code_in_raw_mode(|| {
        let mut tty_device = OpenOptions::new()
            .read(true)
            .write(true)
            .append(true)
            .open("/dev/tty")
            .unwrap();

        let mut buffer = [0; 4]; // read exactly one byte
        let _result = write!(tty_device, "\u{1b}[14t");
        // stdout().flush().unwrap(); // this is critical to make the terminal get the above

        tty_device.read_exact(&mut buffer).unwrap();
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
                tty_device.read_exact(&mut charbuffer).unwrap();
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
    let tty_device = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("/dev/tty")
        .unwrap();

    let tty_device_fd = tty_device.as_raw_fd();

    let mut termios = Termios::from_fd(tty_device_fd).unwrap();
    let old_termios = termios.clone();

    termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(tty_device_fd, TCSANOW, &mut termios).unwrap();
    cfmakeraw(&mut termios);

    let return_value = func();

    tcsetattr(tty_device_fd, TCSANOW, &old_termios).unwrap(); // reset the stdin to the original termios data
    return_value
}
