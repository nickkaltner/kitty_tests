use std::ffi::c_ushort;

use nix::libc::{ioctl, STDOUT_FILENO};

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
