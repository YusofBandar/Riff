use structopt::StructOpt;
use riff::diff::{compare, CompareOptions};

#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to image (jpeg or png) to compare from
    base_path: String,

    /// Path to image (jpeg or png) to compare to
    diff_path: String,

    /// Path to output image (jpeg or png)
    #[structopt(long = "output", short = "o", default_value = "./output.png")]
    output_path: String,

    /// The color of differing pixels in [R, G, B, A] format
    #[structopt(long = "diffColour", short = "c", default_value = "[218, 165, 32, 255]", parse(try_from_str="parse_num_array"))]
    diff_colour: [u32; 4],

    /// Matching threshold, smaller values makes pixel comparison more sensitive
    #[structopt(long = "threshold", short = "t", default_value = "0.1")]
    threshold: f64,

    /// Blending value of unchaged pixels
    #[structopt(long = "alpha", short = "a")]
    alpha: Option<f64>,

    /// The region within base image to compare to in [x, y, width, height] format. Useful when comparing differently sized images
    #[structopt(long="view", parse(try_from_str = "parse_num_array"))]
    view_port: Option<[u32; 4]>
}

fn parse_num_array(array: &str) -> Result<[u32; 4], &'static str> {
    let array = array.trim_start_matches("[")
        .trim_end_matches("]")
        .split(",");

    let mut num_array: [u32; 4] = [255; 4];
    for (i, el) in array.enumerate() {
        num_array[i] = el.trim().parse::<u32>().expect("Argument incorrectly formatted, correct format should be: [a, b, c]");
    }

    Ok(num_array)
}

fn main() {
    let opt = Opt::from_args();

    let base = String::from(opt.base_path);
    let diff = String::from(opt.diff_path);

    let base_img = read_image_from_file(&base);
    let diff_img = read_image_from_file(&diff);

    let options = CompareOptions {
        threshold: opt.threshold,
        alpha: opt.alpha,
        diff_colour: opt.diff_colour,
        view_port: opt.view_port
    };

    let img = compare(&base_img, &diff_img, options);
    match img.save(opt.output_path.clone()) { 
        Ok(_) => (),
        Err(_) => panic!("Could not save output file at {}", opt.output_path)}
}

fn read_image_from_file(path: &String) -> image::DynamicImage {
    match image::open(path) {
        Ok(img) => img,
        Err(_) => panic!("Could not read file at {}", path)
    }
}
