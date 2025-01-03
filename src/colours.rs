use palette::{Srgba, WithAlpha};

pub fn get_colour_gradient(colour_count: usize) -> Vec<u32> {
    use enterpolation::{linear::ConstEquidistantLinear, Curve};
    use palette::LinSrgb;

    let gradient = ConstEquidistantLinear::<f32, _, 3>::equidistant_unchecked([
        LinSrgb::new(0.95, 0.90, 0.30).with_alpha(1.0),
        LinSrgb::new(0.70, 0.10, 0.20).with_alpha(1.0),
        LinSrgb::new(0.00, 0.05, 0.20).with_alpha(1.0),
    ]);

    let taken_colors: Vec<u32> = gradient
        .take(colour_count)
        .map(|x| {
            // maybe there is a better way to do this conversion
            let a: Srgba<u8> = x.into_encoding();
            let b: u32 = a.into();
            b
        })
        .collect();

    taken_colors
}

pub fn get_colour_gradient_stars(colour_count: usize) -> Vec<u32> {
    use enterpolation::{linear::ConstEquidistantLinear, Curve};
    use palette::LinSrgb;

    let gradient = ConstEquidistantLinear::<f32, _, 2>::equidistant_unchecked([
        LinSrgb::new(0.9, 0.90, 0.30).with_alpha(1.0),
        // LinSrgb::new(0.050, 0.05, 0.05).with_alpha(1.0),
        LinSrgb::new(0.050, 0.05, 0.16).with_alpha(1.0),
    ]);

    let taken_colors: Vec<u32> = gradient
        .take(colour_count)
        .map(|x| {
            // maybe there is a better way to do this conversion
            let a: Srgba<u8> = x.into_encoding();
            let b: u32 = a.into();
            b
        })
        .collect();

    taken_colors
}
