use image::Rgba;
use structopt::StructOpt;
use riff::colour_delta::{calculate_pixel_colour_delta, blend, rgb2y, rgba_to_f64};

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to image (jpeg or png) to compare from
    base_path: String,

    /// Path to image (jpeg or png) to compare to
    diff_path: String,

    /// Path to output image (jpeg or png)
    output_path: String,

    /// The color of differing pixels in [R, G, B, A] format
    #[structopt(long="diffColour", default_value="[218, 165, 32, 255]", parse(try_from_str = "parse_num_array"))]
    diff_colour: [u32; 4],

    /// Matching threshold, smaller values makes pixel comparison more sensitive
    #[structopt(long = "threshold", default_value="0.1")]
    threshold: f64,

    /// Blending value of unchaged pixels, 0 alpha disables drawing of base image
    #[structopt(long = "alpha", default_value="0")]
    alpha: f64,

    /// The region within base image to compare to in [x, y, width, height] format. Useful when comparing differently sized images
    #[structopt(long="viewPort", parse(try_from_str = "parse_num_array"))]
    view_port: Option<[u32; 4]>
}

fn parse_num_array(array: &str) -> Result<[u32; 4], &'static str> {
    let array = array.trim_start_matches("[")
        .trim_end_matches("]")
        .split(",");

    let mut num_array: [u32; 4] = [255; 4];
    for (i, el) in array.enumerate() {
        num_array[i] = el.trim().parse::<u32>().unwrap();
    }

    Ok(num_array)
}

fn main() {
    let opt = Opt::from_args();

    let base = String::from(opt.base_path);
    let diff = String::from(opt.diff_path);

    let base_img = read_image_from_file(&base);
    let diff_img = read_image_from_file(&diff);

    let options = CompareOptions{
        threshold: opt.threshold,
        alpha: opt.alpha,
        diff_colour: opt.diff_colour,
        view_port: opt.view_port
    };

    let img = compare(&base_img, &diff_img, options);
    img.save(opt.output_path).unwrap();
}

fn read_image_from_file(path: &String) -> image::DynamicImage {
    image::open(path).unwrap()
}

struct CompareOptions {
    threshold: f64,
    alpha: f64,
    diff_colour: [u32; 4],
    view_port: Option<[u32; 4]>
}

fn compare (base: &image::DynamicImage, diff: &image::DynamicImage, options: CompareOptions) -> image::RgbaImage {
    let CompareOptions { threshold, alpha, diff_colour, view_port } = options;
    let base = base.to_rgba8();
    let diff = diff.to_rgba8();

    let (width, height) = base.dimensions();

    let view_port = match view_port {
        Some(view) => [view[0], view[1], view[2] - 1, view[3] - 1],
        None => [0, 0, width-1, height-1]
    };
    
    let view_port = [view_port[0], view_port[1], view_port[2] - 1, view_port[3] - 1];
    let max_delta = (35215 as f64) * threshold * threshold;

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
                    diff_image.put_pixel(x, y, Rgba([diff_colour[0] as u8, diff_colour[1] as u8, diff_colour[2] as u8, diff_colour[3] as u8]));
                }
            } else if alpha > 0.0  {
                // draw unchaged pixels with specified alpha
                let pixel = rgba_to_f64(&base_pixel[0], &base_pixel[1], &base_pixel[2], &base_pixel[3]);
                let colour = rgb2y(&pixel[0], &pixel[1], &pixel[2]);
                let blended_pixel = blend(&colour, &(alpha * pixel[3] / (255 as f64)));
                diff_image.put_pixel(x, y, Rgba([blended_pixel as u8, blended_pixel as u8, blended_pixel as u8, 255]));
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
