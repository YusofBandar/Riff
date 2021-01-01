use image::{Rgba, Primitive};

fn blend(colour: &f64, alpha: &f64) -> f64 {
    (255 as f64) + (colour - (255 as f64)) * alpha
}

fn rgb2y(r: &f64, g: &f64, b: &f64) -> f64 {
    r * 0.29889531 + g * 0.58662247 + b * 0.11448223
}

fn rgb2i(r: &f64, g: &f64, b: &f64) -> f64 {
    r * 0.59597799 - g * 0.27417610 - b * 0.32180189
}

fn rgb2q(r: &f64, g: &f64, b: &f64) -> f64 {
    r * 0.21147017 - g * 0.52261711 + b * 0.31114694
}

fn blend_semi_transparent_colour(r: &f64, g: &f64, b: &f64, a: &f64) -> [f64; 4] {
    if a < &(255 as f64) {
        [blend(&r, &a), blend(&g, &a), blend(&b, &a), a / 255 as f64]
    } else {
        [r.clone(), g.clone(), b.clone(), a.clone()]
    }
}

fn rgba_to_f64<T: Primitive>(r: &T, g: &T, b: &T, a: &T) -> [f64; 4] {
    let r_64  = match r.to_f64() {
        Some(val) => val,
        None => 0 as f64
    };

    let g_64  = match g.to_f64() {
        Some(val) => val,
        None => 0 as f64
    };

    let b_64  = match b.to_f64() {
        Some(val) => val,
        None => 0 as f64
    };

    let a_64  = match a.to_f64() {
        Some(val) => val,
        None => 0 as f64
    };

    [r_64, g_64, b_64, a_64]
}

pub fn calculate_pixel_colour_delta<T: Primitive>(pixel_a: &Rgba<T>, pixel_b: &Rgba<T>) -> f64 {
    let pixel_a_64 = rgba_to_f64(&pixel_a[0], &pixel_a[1], &pixel_a[2], &pixel_a[3]);
    let pixel_b_64 = rgba_to_f64(&pixel_b[0], &pixel_b[1], &pixel_b[2], &pixel_b[3]);

    let blended_pixel_a = blend_semi_transparent_colour(&pixel_a_64[0], &pixel_a_64[1], &pixel_a_64[2], &pixel_a_64[3]);
    let blended_pixel_b = blend_semi_transparent_colour(&pixel_b_64[0], &pixel_b_64[1], &pixel_b_64[2], &pixel_b_64[3]);

    let y = rgb2y(&blended_pixel_a[0],&blended_pixel_a[1],&blended_pixel_a[2]) - rgb2y(&blended_pixel_b[0],&blended_pixel_b[1],&blended_pixel_b[2]);
    let i = rgb2i(&blended_pixel_a[0],&blended_pixel_a[1],&blended_pixel_a[2]) - rgb2i(&blended_pixel_b[0],&blended_pixel_b[1],&blended_pixel_b[2]);
    let q = rgb2q(&blended_pixel_a[0],&blended_pixel_a[1],&blended_pixel_a[2]) - rgb2q(&blended_pixel_b[0],&blended_pixel_b[1],&blended_pixel_b[2]);

    0.5053 * y * y + 0.299 * i * i + 0.1957 * q * q
}