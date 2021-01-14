use image::Rgba;
use super::colour_delta::{calculate_pixel_colour_delta, blend, rgb2y, rgba_to_f64};

pub struct CompareOptions {
    pub threshold: f64,
    pub alpha: f64,
    pub diff_colour: [u32; 4],
    pub view_port: Option<[u32; 4]>
}

pub fn compare (base: &image::DynamicImage, diff: &image::DynamicImage, options: CompareOptions) -> image::RgbaImage {
    let CompareOptions { threshold, alpha, diff_colour, view_port } = options;
    let base = base.to_rgba8();
    let diff = diff.to_rgba8();

    let (width, height) = base.dimensions();

    let view_port = match view_port {
        Some(view) => [view[0], view[1], view[2] - 1, view[3] - 1],
        None => [0, 0, width - 1, height - 1]
    };
    
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
                let delta = calculate_pixel_colour_delta(&[base_pixel[0], base_pixel[1], base_pixel[2], base_pixel[3]], &[diff_pixel[0], diff_pixel[1], diff_pixel[2], diff_pixel[3]]);
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

#[cfg(test)]
mod tests {
    use image::{DynamicImage, ImageBuffer, RgbImage};
    use super::{compare, CompareOptions};


    #[test]
    fn compares() {
        let options = CompareOptions {
            threshold: 0.1,
            alpha: 0.0,
            diff_colour: [255, 0, 0, 0],
            view_port: None
        };

        let image_a: RgbImage = ImageBuffer::from_vec(3, 1, vec![255, 255, 255, 0, 0, 0, 0, 0, 0]).unwrap();
        let image_b: RgbImage = ImageBuffer::from_vec(3, 1, vec![0, 0, 0, 255, 255, 255, 0, 0, 0]).unwrap();

        let diff = compare(&DynamicImage::ImageRgb8(image_a), &DynamicImage::ImageRgb8(image_b), options);

        assert_eq!(diff.as_raw().len(), 3 * 4);
        assert_eq!(diff.get_pixel(0, 0)[0], 255);
        assert_eq!(diff.get_pixel(1, 0)[0], 255);
    }

    #[test]
    fn compares_with_viewport() {
        let options = CompareOptions {
            threshold: 0.1,
            alpha: 0.0,
            diff_colour: [255, 0, 0, 0],
            view_port: Some([1, 0, 2, 1])
        };

        let image_a: RgbImage = ImageBuffer::from_vec(3, 1, vec![255, 255, 255, 0, 0, 0, 0, 0, 0]).unwrap();
        let image_b: RgbImage = ImageBuffer::from_vec(3, 1, vec![0, 0, 0, 255, 255, 255, 0, 0, 0]).unwrap();

        let diff = compare(&DynamicImage::ImageRgb8(image_a), &DynamicImage::ImageRgb8(image_b), options);

        assert_eq!(diff.as_raw().len(), 3 * 4);
        assert_eq!(diff.get_pixel(0, 0)[0], 0);
        assert_eq!(diff.get_pixel(1, 0)[0], 255);
    }
}
