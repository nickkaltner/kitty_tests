use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use core::str;
use log::warn;
use palette::{Srgba, WithAlpha};

pub mod drawing;
pub mod encoding;
pub mod misc;
pub mod terminal_details;

const SPARKLINE_HEIGHT: usize = 20;

fn main() {
    let mut random_list_length: u32 = rand::random();
    random_list_length %= 40 + 20;

    let mut random_list_min: u32 = rand::random();
    random_list_min %= 8; // 0-7

    let mut list: Vec<u32> = Vec::new();

    for _i in 0..random_list_length {
        let mut rand: u32 = rand::random();
        rand %= 20;
        rand += random_list_min;

        list.push(rand);
    }

    let config = SparklineConfig {
        suppress_text: true,
        generate_random_data: true,
    };

    // test of flat sparkline
    // sparkline(vec![
    //     10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    //     10, 10, 10, 10, 10, 10, 10,
    // ]);

    sparkline(list, config);
}

pub struct SparklineConfig {
    pub suppress_text: bool,
    pub generate_random_data: bool,
}

fn sparkline(values: Vec<u32>, config: SparklineConfig) -> String {
    if values.len() == 0 {
        panic!("no values");
    }
    let mut useful_values = values.clone();

    // get column count from terminal size
    let (terminal_column_count, _) = terminal_details::get_window_size();

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

    let max_value = *(useful_values.iter().max().unwrap());
    let min_value = *(useful_values.iter().min().unwrap());

    if !config.suppress_text {
        println!(
            "min {} max {} value_count {}",
            min_value, max_value, value_count
        );
    }

    // get colours

    use enterpolation::{linear::ConstEquidistantLinear, Curve};
    use palette::LinSrgb;

    let gradient = ConstEquidistantLinear::<f32, _, 3>::equidistant_unchecked([
        LinSrgb::new(0.00, 0.05, 0.20).with_alpha(1.0),
        LinSrgb::new(0.70, 0.10, 0.20).with_alpha(1.0),
        LinSrgb::new(0.95, 0.90, 0.30).with_alpha(1.0),
    ]);

    let taken_colors: Vec<_> = gradient.take(SPARKLINE_HEIGHT).rev().collect();

    //let mut zz: Srgba<u8> = taken_colors[1].into_encoding();

    // create background image - 1 pixel wide for each value, 20 pixels high, 4 bytes per pixel

    let image_size = SPARKLINE_HEIGHT * value_count;
    let mut data: Vec<u32> = Vec::with_capacity(image_size);
    data.resize(image_size, 0xFF0000FF); // alpha, blue, green, red - unsure why this is reversed

    for row in 0..SPARKLINE_HEIGHT as usize {
        let colour: Srgba<u8> = taken_colors[row as usize].into_encoding();
        // data[i * value_count] = 0xFF000000;
        for value_position in 0..value_count {
            let rar = u32::from_le_bytes(colour.into());
            let position = (row * value_count) + value_position as usize;
            // println!("position {}, data {}", position, data.len());
            if position >= data.len() {
                println!(
                    "position {} is out of bounds (data.len is {})",
                    position,
                    data.len()
                );
            } else {
                data[position] = rar;
            }
        }
    }

    // let mut sparkline = String::new();

    let range = (max_value - min_value) as f32;

    let percentage_divisor = 100.0 / SPARKLINE_HEIGHT as f32;

    for i in 0..value_count {
        let value = values[i];
        let mut percentage = ((value as f32 - min_value as f32) / range) * 100.0;
        // to account for the case where the value is the same as the max
        if percentage.is_infinite() || percentage.is_nan() {
            percentage = 100.0;
        }
        let bar_count = (percentage / percentage_divisor) as u32;
        // println!(
        //     "value {} min {} max {} range {} percentage {} bar_count {}",
        //     value, min_value, max_value, range, percentage, bar_count
        // );

        let bars_to_remove = SPARKLINE_HEIGHT as u32 - bar_count;

        // println!(
        //     "value {} percentage {} bar_count {} bars_to_remove {}",
        //     value, percentage, bar_count, bars_to_remove
        // );

        for j in 0..bars_to_remove {
            data[j as usize * value_count + i] = 0xFF000000; // black out those bars
        }
    }

    // println!("full_array {:?}", data);

    // i wonder if this screws up the order of the data - it seems to be coming out reversed eg. Instead of RGBA, it's ABGR
    let (_x, data2, _z) = unsafe { data.align_to::<u8>() };

    let output = BASE64_STANDARD.encode(data2);

    let ansi_output = format!(
        "\u{1b}_Ga=T,q=2,f=32,s={},v={},c={},r=1;{}\u{1b}\\",
        value_count, SPARKLINE_HEIGHT, value_count, output
    );

    if config.suppress_text {
        println!("{}", ansi_output);
    } else {
        println!("sparkline: {}", ansi_output);
        println!("sparkline: 123456789012345678901234567890123456789012345678901234567890");
    }
    ansi_output
}
