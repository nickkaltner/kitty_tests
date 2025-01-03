pub mod colours;
pub mod drawing;
pub mod encoding;
pub mod misc;
pub mod terminal_details;

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

    let config = drawing::SparklineConfig {
        suppress_text: true,
        generate_random_data: true,
    };

    // test of flat sparkline
    // sparkline(vec![
    //     10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
    //     10, 10, 10, 10, 10, 10, 10,
    // ]);

    drawing::sparkline(list, config);
}
