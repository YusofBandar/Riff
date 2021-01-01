use image::Rgba;
use riff::colour_delta::calculate_pixel_colour_delta;

fn main() {
    let base = String::from("./images/tiger.jpeg");
    let diff = String::from("./images/tiger-2.jpeg");

    let base_img = read_image_from_file(&base);
    let diff_img = read_image_from_file(&diff);

    // golden-rod!!
    let diff_colour = [218 as u8, 165 as u8, 32 as u8, 255 as u8];
    let view_port = [0, 0, 1000, 419];

    let img = compare(&base_img, &diff_img, diff_colour, view_port);
    img.save("diff.png").unwrap();
}

fn read_image_from_file(path: &String) -> image::DynamicImage {
    image::open(path).unwrap()
}

fn compare (base: &image::DynamicImage, diff: &image::DynamicImage, diff_colour: [u8; 4], view_port: [u32; 4]) -> image::RgbaImage {
    let base = base.to_rgba8();
    let diff = diff.to_rgba8();
    
    let view_port = [view_port[0], view_port[1], view_port[2] - 1, view_port[3] - 1];
    let max_delta = (35215 as f64) * 0.1 * 0.1;

    let (width, height) = base.dimensions();

    let mut diff_image: image::RgbaImage = image::ImageBuffer::new(width, height);

    for (x, y, base_pixel) in base.enumerate_pixels() {
        if is_past_viewport(x, y, view_port) {
            break;
        }

        if is_within_viewport(x, y, view_port) {
            let diff_pixel = diff.get_pixel(x, y);

            // if pixels are the same don't bother to calculate delta
            if base_pixel[0] != diff_pixel[0] || base_pixel[1] != diff_pixel[1] || base_pixel[2] != diff_pixel[2] || base_pixel[3] != diff_pixel[3] {
                let delta = calculate_pixel_colour_delta(base_pixel, diff_pixel);
                if delta > max_delta {
                    diff_image.put_pixel(x, y, Rgba(diff_colour));
                }
            }
        }
    }

    diff_image
}

fn is_past_viewport(x: u32, y: u32, view_port: [u32; 4]) -> bool {
    x >= view_port[0] + view_port[2] && y > view_port[1] + view_port[3]
}

fn is_within_viewport(x: u32, y: u32, view_port: [u32; 4]) -> bool {
    x >= view_port[0] && x <= view_port[0] + view_port[2] &&
     y >= view_port[1] && y <= view_port[3]
}
