use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use core::str;
use log::warn;
use nix::libc::{ioctl, STDOUT_FILENO};
use palette::{Srgba, WithAlpha};
use rand::*;
use std::ffi::c_ushort;
use std::io;
use std::io::Read;
use std::os::fd::AsRawFd;
use termios::*;

fn main() {
    //println!("Hello, world!");

    //println!("{}", text_with_underline("Hello, world!"));

    // println!(
    //     "{}",
    //     text_with_red_underline("Is this a spelling mistake?!")
    // );

    //get_screen_size();

    // println!("{}", encode_graphics(20, 25));

    // println!("terminal dimensions: {:?}", get_window_size());

    let mut rand: u32 = rand::random();
    rand %= 40 + 20;

    let mut list: Vec<u32> = Vec::new();

    for i in 0..rand {
        let mut rand: u32 = rand::random();
        rand %= 20;

        list.push(rand);
    }

    sparkline(vec![
        10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
        10, 10, 10, 10, 10, 10, 10,
    ]);

    sparkline(list);

    // sparkline(vec![
    //     0, 2, 3, 4, 5, 6, 27, 28, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    // ]);
}

fn _text_with_underline(string: &str) -> String {
    let mut result = String::new();

    let out = "\u{1b}[4m";

    result.push_str(out);
    result.push_str(string);
    result.push_str(out);

    result
}

fn text_with_red_underline(string: &str) -> String {
    format!("\u{1b}[4:3m\u{1b}[58;5;196m{}\u{1b}[0m\u{1b}[59m", string)
}

fn _get_screen_size() -> (u16, u16) {
    // let mut size = (0, 0);
    // let mut stdout = std::io::stdout();

    // let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work
    //                // on /dev/stdin or /dev/tty
    // let termios = Termios::from_fd(stdin).unwrap();

    // let mut termios_backup = Termios::from_fd(stdin).unwrap();

    // tcgetattr(stdin, &mut termios_backup).unwrap();

    // let mut new_termios = termios.clone();
    // new_termios.c_lflag &= !(ICANON | ECHO);
    // tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    // cfmakeraw(&mut new_termios);
    // // size
    // print!("\u{1b}[14t");

    // let mut reader = io::stdin();
    // let mut buffer = [0; 1]; // read exactly one byte
    // print!("Hit a key! ");
    // reader.read_exact(&mut buffer).unwrap();
    // println!("You have hit: {:?}", buffer);

    // for i in io::stdin().lock().bytes() {
    //     println!("read byte {}", i.unwrap());
    // }

    // tcsetattr(stdin, TCSANOW, &termios).unwrap();

    println!("using fd {}", io::stdin().as_raw_fd());
    let stdin = 0; // couldn't get std::os::unix::io::FromRawFd to work
                   // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone(); // make a mutable copy of termios
                                           // that we will modify
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    cfmakeraw(&mut new_termios);
    let mut reader = io::stdin().lock();
    let mut buffer = [0; 1]; // read exactly one byte
    print!("Hit a key! ");
    print!("\u{1b}[14t");

    reader.read_exact(&mut buffer).unwrap();
    println!("You have hit: {:?}", buffer);
    tcsetattr(stdin, TCSANOW, &termios).unwrap(); // reset the stdin to
                                                  // original termios data

    (1, 1)
}

fn white_line(length: u16) -> String {
    let four_byte_length: usize = length as usize * 4;
    // format r, g, b, a
    // let whitergba: [u8; 4] = [221, 160, 221, 255];
    let stripergba: [u8; 16] = [
        221, 160, 221, 255, 0, 20, 0, 255, 0, 20, 0, 255, 221, 160, 221, 255,
    ];

    //let image_length = stripergba.len()

    let array = vec![stripergba; length as usize / 4];
    let full_array = array.as_flattened();

    //let full_array = array.iter().map(|x| *x).collect::<Vec<u8>>();
    // println!("length {}", full_array.len());
    // println!("array {:?}", full_array);

    assert!(
        full_array.len() == four_byte_length,
        "{} == {}",
        full_array.len(),
        four_byte_length
    );

    let str = BASE64_STANDARD.encode(full_array);

    // println!("str {}", str);

    str
}

fn encode_graphics(width: u16, height: u16) -> String {
    let data = white_line(width * height);

    // https://sw.kovidgoyal.net/kitty/graphics-protocol/#a-minimal-example

    // println!(
    //     "encoding: _Ga=T,q=2,f=32,s={},v={},x=2;{}",
    //     width, height, data
    // );

    // to center, \u{1b}[64C beforehand

    // c=4 is the number of columsn to take up

    let output = format!(
        "\u{1b}_Ga=T,q=2,f=32,s={},v={},Y=2,c=4;{}\u{1b}\\",
        width, height, data
    );

    output
}

#[repr(C)]
pub struct WinSize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

fn get_window_size() -> (u16, u16) {
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

fn sparkline(values: Vec<u32>) -> String {
    let mut useful_values = values.clone();

    // get column count from terminal size
    let (terminal_column_count, _) = get_window_size();

    if useful_values.len() > terminal_column_count as usize {
        warn!(
            "value count {} is greater than terminal terminal_column_count {}",
            useful_values.len(),
            terminal_column_count
        );

        // truncate values to the last terminal_column_count values
        useful_values = useful_values.split_off(terminal_column_count as usize);
    }
    let value_count = useful_values.len();
    let max_value = *useful_values.iter().max().unwrap();
    let min_value = *useful_values.iter().min().unwrap();
    println!(
        "min {} max {} value_count {}",
        min_value, max_value, value_count
    );

    // get colours

    use enterpolation::{linear::ConstEquidistantLinear, Curve};
    use palette::LinSrgb;

    let gradient = ConstEquidistantLinear::<f32, _, 3>::equidistant_unchecked([
        LinSrgb::new(0.00, 0.05, 0.20).with_alpha(1.0),
        LinSrgb::new(0.70, 0.10, 0.20).with_alpha(1.0),
        LinSrgb::new(0.95, 0.90, 0.30).with_alpha(1.0),
    ]);

    let taken_colors: Vec<_> = gradient.take(20).collect();

    //let mut zz: Srgba<u8> = taken_colors[1].into_encoding();

    // create background image - 1 pixel wide for each value, 20 pixels high, 4 bytes per pixel

    let image_size = 20 * value_count;
    let mut data: Vec<u32> = Vec::with_capacity(image_size);
    data.resize(image_size, 0xAAFF6666);
    for i in 0..20 {
        let colour: Srgba<u8> = taken_colors[i as usize].into_encoding();
        // data[i * value_count] = 0xFF000000;
        for j in 0..value_count {
            let rar = u32::from_le_bytes(colour.into());
            let position = i * 20 + j as usize;
            //println!("position {}, data {}", position, data.len());
            if position >= data.len() {
                //println!("position {} is out of bounds", position);
            } else {
                data[position] = rar;
            }
        }
    }

    // let mut sparkline = String::new();

    let range = (max_value - min_value) as f32;

    for i in 0..value_count {
        let value = values[i];
        let mut percentage = ((value as f32 - min_value as f32) / range) * 100.0;
        // to account for the case where the value is the same as the max
        if percentage.is_infinite() {
            percentage = 100.0;
        }
        let bar_count = (percentage / 5.0) as u32;
        println!(
            "value {} min {} max {} range {} percentage {} bar_count {}",
            value, min_value, max_value, range, percentage, bar_count
        );

        let bars_to_remove = 20 - bar_count;

        // println!(
        //     "value {} percentage {} bar_count {} bars_to_remove {}",
        //     value, percentage, bar_count, bars_to_remove
        // );

        for j in 0..bars_to_remove {
            // println!("removing from point {}", i * value_count + j as usize);
            data[j as usize * value_count + i] = 0xFF000000;
        }

        // for _ in 0..bar_count {
        //     sparkline.push('█');
        // }
    }

    // println!("full_array {:?}", data);

    let (_x, data2, _z) = unsafe { data.align_to::<u8>() };

    let output = BASE64_STANDARD.encode(data2);

    let ansi_output = format!(
        "\u{1b}_Ga=T,q=2,f=32,s={},v={},c={};{}\u{1b}\\",
        value_count, 20, value_count, output
    );

    // let ansi_output = format!(
    //     "\u{1b}_Ga=T,q=2,f=32,s={},v={};{}\u{1b}\\",
    //     value_count, 20, output
    // );

    println!("sparkline: {}", ansi_output);
    println!("sparkline: 123456789012345678901234567890");

    "eek".to_string()
}
