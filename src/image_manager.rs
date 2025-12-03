use std::path::PathBuf;
use image::{DynamicImage, ImageReader};
use crate::filters;

#[derive(Default)]
pub struct ImageConfig {
    pub picked_path: Option<PathBuf>,
    pub orig_img: DynamicImage,
    pub downscaled_display_img: DynamicImage,
    
    pub max_generated_img: usize,

    pub ascii_img: Option<DynamicImage>,
    pub sobel_img: Option<DynamicImage>,
    pub gauss_img: Option<DynamicImage>,
    pub edges: Option<Vec<Vec<usize>>>
}

impl ImageConfig {
    pub fn change_image(&mut self, path: PathBuf, down_scale_factor: u32) -> (bool, bool, bool) {
        self.picked_path = Some(path);
        self.orig_img = match ImageReader::open(self.picked_path.clone().unwrap()) {
            Ok(reader) => match reader.with_guessed_format() {
                Ok(reader) => match reader.decode() {
                    Ok(img) => img,
                    Err(e) => panic!("Failed to decode image: {:?}", e),
                },
                Err(e) => panic!("Failed to guess image format: {:?}", e),
            },
            Err(e) => panic!("Failed to open image: {:?}", e),
        };

        // Crops the image, so that division works well without errors
        let cropped_w = if self.orig_img.width() % down_scale_factor != 0 {
            let crop_pixels = self.orig_img.width() % down_scale_factor;
            self.orig_img = self.orig_img.crop(0, 0, self.orig_img.width()-crop_pixels, self.orig_img.height());
            true
        } else {
            false
        };
        
        let cropped_h = if self.orig_img.height() % down_scale_factor != 0 {
            let crop_pixels = self.orig_img.height() % down_scale_factor;
            self.orig_img = self.orig_img.crop(0, 0, self.orig_img.width(), self.orig_img.height()-crop_pixels);
            true
        } else {
            false
        };

        (true, cropped_w, cropped_h)
    }
    pub fn generate_gauss(&mut self, sigma_one: f32, sigma_two: f32, threshold: i32, tau: f32) {
        let gauss = filters::gaussian::gaussian_diff(&self.orig_img, sigma_one, sigma_two, threshold, tau);
        self.gauss_img = Some(DynamicImage::ImageRgb8(gauss));
        self.sobel_img = None;
        self.ascii_img = None;
    }

    pub fn generate_sobel(&mut self, down_scale_factor: u32, edge_threshold: i32) {
        let (sobel, edges) = filters::edge_detect::edge_filter(&self.gauss_img.clone().unwrap(), &self.orig_img, down_scale_factor, edge_threshold);
        self.sobel_img = Some(DynamicImage::ImageRgb8(sobel));
        self.edges = Some(edges);
        self.ascii_img = None;
    }

    pub fn generate_ascii(&mut self, font: &PathBuf, down_scale_factor: u32, charset: &Vec<String>, upscale_factor: u32, gamma: f32) {
        let ascii = filters::ascii::to_ascii_image(&self.orig_img.clone(), font, down_scale_factor, &self.edges.clone().unwrap(), &charset, upscale_factor, gamma);
        self.ascii_img = Some(DynamicImage::ImageRgb8(ascii));
    }
}
