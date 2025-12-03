use std::path::PathBuf;
use eframe::egui;
use egui_notify::Anchor::BottomRight;
use egui_notify::Toasts;


use crate::ui;
use crate::image_manager;

#[derive(Default)]
pub struct ParameterConfig {
    pub sigma_one: i32,
    pub sigma_two: i32,
    pub threshold: i32,
    pub edge_threshold: i32,
    pub tau: f32,
    pub gamma: f32,
}

#[derive(Default)]
pub struct UiConfig {
    pub image_type: usize, // 0 - Original image, 1 - Ascii image, 2 - Sobel image, 3 - Gaussian image
    
    pub down_scale_factors: Vec<u32>,
    pub up_scale_factors: Vec<u32>,
    pub down_scale_factor_id: u32,
    pub up_scale_factor_id: u32,

    pub charset_text: String,
    pub charset: Vec<String>,
}

#[derive(Default)]
pub struct AsciiApp {
    pub font: PathBuf,
    pub toasts: Toasts,
    pub changed: bool,

    pub zoom: f32,
    pub pan: egui::Vec2,

    pub image_config: image_manager::ImageConfig,
    pub parameter_config: ParameterConfig,
    pub ui_config: UiConfig
}

impl AsciiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        ui::utils::configure_font(&cc.egui_ctx);
        let default_image_path = std::env::current_dir().unwrap().join("images\\pipe.jpg");
        let mut app = Self {
            font: std::env::current_dir().unwrap().join("Bescii-Mono.ttf"),
            toasts: Toasts::default().with_anchor(BottomRight),
            changed: true,

            zoom: 1.0,
            pan: egui::Vec2::ZERO,

            image_config: image_manager::ImageConfig {
                picked_path: Some(PathBuf::from(default_image_path.clone())),
                orig_img: image::open(default_image_path).unwrap(),
                max_generated_img: 4,
                ..Default::default()
            },

            parameter_config: ParameterConfig {
                sigma_one: 7,
                sigma_two: 20,
                threshold: 10,
                edge_threshold: 4,
                tau: 0.9,
                gamma: 2.5
            },

            ui_config: UiConfig {
                image_type: 0,
                
                down_scale_factors: vec![2, 4, 8, 16], // Downscale factors to decrease the img size
                up_scale_factors: vec![1, 2, 4, 8, 16], // Upscale factors for the final image
                down_scale_factor_id: 0,
                up_scale_factor_id: 0,
                
                charset_text: String::from(" , ., ;, c, o, P, O, ?, @, ■"),
                charset: [" ", ".", ";", "c", "o", "P", "O", "?", "@", "■"].iter().map(|&x| x.to_string()).collect(),
            }
        };

        app.image_config.downscaled_display_img = app.image_config.orig_img.clone();
        app
    }
}

impl AsciiApp {
    // Updates the image, based on the currently selected display image
    // Checks if you have to regenerate the images
    // 0 - Original image, 1 - Ascii image, 2 - Sobel image, 3 - Gaussian image
    pub fn update_images(&mut self) {

        let generate_from = if self.changed {
            4
        } else {
            self.image_config.max_generated_img
        };

        // Generate each image for each step. Ascii needs all steps, gauss needs itself
        if self.ui_config.image_type <= generate_from && self.ui_config.image_type != 0 {
            for i in (self.ui_config.image_type..generate_from).rev() {
                match i {
                    1 => { // Ascii image
                        println!("Generating Ascii image");
                        self.image_config.generate_ascii(
                            &self.font,
                            self.get_down_scale_factor(),
                            &self.ui_config.charset,
                            self.get_upscale_factor(),
                            self.parameter_config.gamma
                        );
                    },
                    2 => { // Sobel image
                        println!("Generating Sobel image");
                        self.image_config.generate_sobel(
                            self.get_down_scale_factor(),
                            self.parameter_config.edge_threshold
                        );
                    },
                    3 => { // Gaussian image
                        println!("Generating Gaussian image");
                        self.image_config.generate_gauss(
                            self.parameter_config.sigma_one as f32,
                            self.parameter_config.sigma_two as f32,
                            self.parameter_config.threshold,
                            self.parameter_config.tau
                        );
                    },
                    _ => {},
                }
            }

            self.image_config.max_generated_img = self.ui_config.image_type;
        }

        self.changed = false;
        // Set the display image
        self.set_display_image();
    }

    // Downscales the current image that should be displayed
    fn set_display_image(&mut self) {

        let disp_img = match self.ui_config.image_type {
            0 => {
                self.image_config.orig_img.clone()
            },
            1 => {

                self.image_config.ascii_img.clone().unwrap()
            },
            2 => {

                self.image_config.sobel_img.clone().unwrap()
            },
            3 => {
                self.image_config.gauss_img.clone().unwrap()
            },
            _ => {
                self.image_config.orig_img.clone()
            }
        };

        self.image_config.downscaled_display_img = disp_img;
    }
    
    pub fn get_down_scale_factor(&self) -> u32 {
        self.ui_config.down_scale_factors[self.ui_config.down_scale_factor_id as usize]
    }

    pub fn get_upscale_factor(&self) -> u32 {
        self.ui_config.up_scale_factors[self.ui_config.up_scale_factor_id as usize]
    }

    pub fn check_charset_correctness(&mut self) -> bool {
        let split_commas_remove_space: Vec<String> = self.ui_config.charset_text
            .split(',')
            .map(|x| if x.trim().is_empty() || x.chars().all(|c| c.is_whitespace()) {
                " ".to_string()  // Single space for any number of spaces
            } else {
                x.trim().to_string()
            }).collect();
        if split_commas_remove_space.iter().all(|x| x.chars().count() == 1) {
            self.ui_config.charset = split_commas_remove_space;
            true
        } else {
            false
        }
    }
}

impl eframe::App for AsciiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui::ui::top_panel(ctx);
        ui::ui::bottom_panel(ctx, self);
        ui::ui::central_panel(ctx, self);

        ui::utils::preview_files_being_dropped(ctx, self);

        // Collect dropped files:
        ctx.input(|i| {
            if !i.raw.dropped_files.is_empty() {
                self.image_config.picked_path = Some(i.raw.dropped_files[0].path.clone().unwrap());
            }
        });

        self.toasts.show(ctx);
    }
}