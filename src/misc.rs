use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use std::io;
use std::io::Read;
use std::os::fd::AsRawFd;
use termios::*;

fn _text_with_underline(string: &str) -> String {
    let mut result = String::new();

    let out = "\u{1b}[4m";

    result.push_str(out);
    result.push_str(string);
    result.push_str(out);

    result
}

fn _text_with_red_underline(string: &str) -> String {
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

fn _white_line(length: u16) -> String {
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

fn _encode_graphics(width: u16, height: u16) -> String {
    let data = _white_line(width * height);

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
