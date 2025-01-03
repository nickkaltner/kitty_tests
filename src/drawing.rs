use crate::colours;
use crate::encoding;
use crate::terminal_details;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use log::warn;

const SPARKLINE_HEIGHT: usize = 20;

pub struct SparklineConfig {
    pub suppress_text: bool,
    pub generate_random_data: bool,
}

pub fn sparkline(values: Vec<u32>, config: SparklineConfig) -> String {
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

    let taken_colours = colours::get_colour_gradient(SPARKLINE_HEIGHT);
    //let taken_colours = colours::get_colour_gradient_stars(SPARKLINE_HEIGHT);

    //let mut zz: Srgba<u8> = taken_colors[1].into_encoding();

    // create background image - 1 pixel wide for each value, 20 pixels high, 4 bytes per pixel

    let image_size = SPARKLINE_HEIGHT * value_count;
    let mut data: Vec<u32> = Vec::with_capacity(image_size);
    data.resize(image_size, 0x000000FF); // RGBA (black)

    for row in 0..SPARKLINE_HEIGHT as usize {
        let pixel_colour: u32 = taken_colours[row as usize];
        // data[i * value_count] = 0xFF000000;
        for value_position in 0..value_count {
            let position = (row * value_count) + value_position as usize;
            // println!("position {}, data {}", position, data.len());
            if position >= data.len() {
                println!(
                    "position {} is out of bounds (data.len is {})",
                    position,
                    data.len()
                );
            } else {
                data[position] = pixel_colour;
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
            data[j as usize * value_count + i] = 0x000000FF; // black out those bars
        }
    }

    // println!("full_array {:?}", data);

    let data_as_8bit_vec = encoding::convert_data_format(data);

    let output = BASE64_STANDARD.encode(data_as_8bit_vec);

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

//  https://gamedev.stackexchange.com/questions/176036/how-to-draw-a-smoother-solid-fill-circle
// TODO: pass in a reference to a graphics context eg. an rgba buffer

// fn _draw_circle(x: u16, y: u16, radius: u16, colour: u32) {
//     let mut x = x;
//     let mut y = y;
//     let radius = radius;
//     let colour = colour;

//     // This code is totally wrong
//     for i in 0..radius {
//         for j in 0..radius {
//             // let x = x + i;
//             // let y = y + j;
//             // let colour = colour;
//             // println!("x {} y {} colour {}", x, y, colour);
//             // data[y as usize * SPARKLINE_HEIGHT as usize + x as usize] = colour;
//         }
//     }
// }

// fn _draw_circle_arc(x: u16, y: u16, radius: u16, start_angle: u16, end_angle: u16, colour: u32) {
//     //let mut x = x;
//     //let mut y = y;
//     let radius = radius;
//     let start_angle = start_angle;
//     let end_angle = end_angle;
//     let colour = colour;

//     // This code is totally wrong
//     for i in 0..radius {
//         for j in 0..radius {
//             // let x = x + i;
//             // let y = y + j;
//             // let colour = colour;
//             // println!("x {} y {} colour {}", x, y, colour);
//             // data[y as usize * SPARKLINE_HEIGHT as usize + x as usize] = colour;
//         }
//     }
// }
