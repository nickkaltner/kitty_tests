//  https://gamedev.stackexchange.com/questions/176036/how-to-draw-a-smoother-solid-fill-circle
// TODO: pass in a reference to a graphics context eg. an rgba buffer

fn draw_circle(x: u16, y: u16, radius: u16, colour: u32) {
    let mut x = x;
    let mut y = y;
    let radius = radius;
    let colour = colour;

    // This code is totally wrong
    for i in 0..radius {
        for j in 0..radius {
            // let x = x + i;
            // let y = y + j;
            // let colour = colour;
            // println!("x {} y {} colour {}", x, y, colour);
            // data[y as usize * SPARKLINE_HEIGHT as usize + x as usize] = colour;
        }
    }
}

fn draw_circle_arc(x: u16, y: u16, radius: u16, start_angle: u16, end_angle: u16, colour: u32) {
    //let mut x = x;
    //let mut y = y;
    let radius = radius;
    let start_angle = start_angle;
    let end_angle = end_angle;
    let colour = colour;

    // This code is totally wrong
    for i in 0..radius {
        for j in 0..radius {
            // let x = x + i;
            // let y = y + j;
            // let colour = colour;
            // println!("x {} y {} colour {}", x, y, colour);
            // data[y as usize * SPARKLINE_HEIGHT as usize + x as usize] = colour;
        }
    }
}
