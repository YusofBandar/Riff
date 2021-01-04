use structopt::StructOpt;
use riff::diff::{compare, CompareOptions};

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
