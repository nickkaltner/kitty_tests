use std::io::{self, stdin, BufRead, IsTerminal};

pub mod colours;
pub mod drawing;
pub mod encoding;
pub mod misc;
pub mod terminal_details;

fn main() {
    let config = drawing::SparklineConfig {
        suppress_text: true,
        generate_random_data: true,
    };

    let mut list: Vec<f64> = Vec::new();

    if stdin().is_terminal() {
        eprintln!("detected terminal, will generate random numbers for demo purposes");

        let (terminal_width, terminal_height) =
            terminal_details::get_terminal_dimensions_in_pixels().unwrap();

        eprintln!(
            "terminal dimensions: {}x{}",
            terminal_width, terminal_height
        );

        let mut random_list_length: u32 = rand::random();
        random_list_length %= 40 + 20;

        let mut random_list_min: u32 = rand::random();
        random_list_min %= 8; // 0-7

        for _i in 0..random_list_length {
            let mut rand: f64 = rand::random();
            rand %= 20 as f64;
            rand += random_list_min as f64;

            list.push(rand);
        }
    } else {
        eprintln!("detected pipe, will read newline separated numbers from stdin");

        let stdin = io::stdin();
        let handle = stdin.lock();

        handle.lines().for_each(|line| match line {
            Ok(line) => {
                let number = line.parse::<f64>();
                match number {
                    Ok(number) => {
                        list.push(number);
                    }
                    Err(e) => {
                        eprintln!("error: {:?} with line {:?}", e, line);
                    }
                }
            }
            Err(e) => {
                println!("error: {:?}", e);
            }
        });
    }

    // test of flat sparkline
    // sparkline(vec![
    //     10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    //     10, 10, 10, 10, 10, 10, 10,
    // ]);

    drawing::sparkline(list, config);
}
