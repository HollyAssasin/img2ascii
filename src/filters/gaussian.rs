use image::{DynamicImage, GenericImageView, RgbImage};
use libblur::{gaussian_blur_image, EdgeMode, ConvolutionMode, ThreadingPolicy};


pub fn gaussian_diff(img: &DynamicImage, sigma_one: f32, sigma_two: f32, threshold: i32, tau: f32) -> RgbImage {
    let grayscale_image = img.to_luma8();

    // Calculate appropriate kernel sizes based on sigmas
    // A common rule of thumb is kernel size = 2 * ceil(3 * sigma) + 1
    let kernel_size_one = (2.0 * (3.0 * sigma_one).ceil() + 1.0) as u32;
    let kernel_size_two = (2.0 * (3.0 * sigma_two).ceil() + 1.0) as u32;

    let first_gaus = gaussian_blur_image(
        DynamicImage::from(grayscale_image.clone()),
        kernel_size_one,
        sigma_one,
        EdgeMode::Clamp,
        ConvolutionMode::FixedPoint,
        ThreadingPolicy::Adaptive,
    ).unwrap();

    let second_gaus = gaussian_blur_image(
        DynamicImage::from(grayscale_image.clone()),
        kernel_size_two,
        sigma_two,
        EdgeMode::Clamp,
        ConvolutionMode::FixedPoint,
        ThreadingPolicy::Adaptive,
    ).unwrap();

    let mut difference = RgbImage::new(grayscale_image.width(), grayscale_image.height());
    for y in 0..grayscale_image.height() {
        for x in 0..grayscale_image.width() {
            let first_val = first_gaus.get_pixel(x, y).0[0] as i32;
            let second_val = (tau * second_gaus.get_pixel(x, y).0[0] as f32) as i32;
            let diff = first_val - second_val;
            let pixel_value = if diff < threshold { 0 } else { 255 };
            difference.put_pixel(x, y, image::Rgb([pixel_value as u8, pixel_value as u8, pixel_value as u8]));
        }
    }

    difference
}