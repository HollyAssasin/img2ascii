use eframe::egui;
use egui::{FontId, Id, RichText, Vec2};
use std::time::Duration;
use imageproc::drawing::Canvas;
use crate::ui;
use crate::ui::utils;

pub fn top_panel(ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(12.0);
                ui.label(
                    RichText::new("Image to Ascii").font(FontId::proportional(18.0)),
                );
                ui.add_space(6.0);
            });
        });
    });
}

pub fn bottom_panel(ctx: &egui::Context, app: &mut ui::app::AsciiApp) {
    egui::TopBottomPanel::bottom("bottom_panel")
        .min_height(ctx.screen_rect().height() / 5.)
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading("Image Control");
                ui.add_space(15.);
                ui.columns(7, |cols| {
                    cols[1].vertical_centered(|ui| {
                        if ui
                            .add(egui::Button::new("Change Image").min_size(Vec2::from([
                                ui.available_width() * 0.95,
                                ui.available_height() * 0.5,
                            ])))
                            .clicked()
                        {
                            if let Some(path) = rfd::FileDialog::new().pick_file() {
                                let (res, cropped_w, cropped_h) = app.image_config.change_image(path, *app.ui_config.down_scale_factors.last().unwrap());
                                if res {
                                    app.toasts.success("Changed image!").duration(Option::from(Duration::from_secs(1)));
                                    if cropped_w || cropped_h {
                                        app.toasts.warning("Cropped image!").duration(Option::from(Duration::from_secs(1)));
                                    }
                                    app.changed = true;
                                    app.update_images();
                                } else {
                                    app.toasts.error("Could not change the image").duration(Option::from(Duration::from_secs(3)));
                                }
                            }
                        }
                    });
                    cols[3].vertical_centered(|ui| {
                        let response = ui.add(egui::Button::new("Change asciis").min_size(Vec2::from([
                            ui.available_width() * 0.95,
                            ui.available_height() * 0.5,
                        ])));
                        let popup_id = Id::new("popup_id");
                        
                        if response.clicked() {
                            ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                        }

                        let below = egui::AboveOrBelow::Above;
                        let close_on_click_outside = egui::popup::PopupCloseBehavior::IgnoreClicks;
                        egui::popup::popup_above_or_below_widget(ui, popup_id, &response, below, close_on_click_outside, |ui| {
                            ui.set_min_width(app.ui_config.charset.len() as f32 * 37.0);
                            ui.set_min_height(150.0);
                            ui.add_space(10.0);
                            ui.label("Chars separated by comma");
                            ui.add_space(20.0);
                            if ui.add(egui::TextEdit::singleline(&mut app.ui_config.charset_text).hint_text("Write something here")).lost_focus() {
                                if app.check_charset_correctness() {
                                    app.ui_config.charset_text = app.ui_config.charset.join(", ");
                                    app.changed = true;
                                    app.update_images();
                                    app.toasts.success("Changed charset!").duration(Option::from(Duration::from_secs(1)));
                                } else {
                                    app.toasts.error("Invalid charset!").duration(Option::from(Duration::from_secs(3)));
                                }
                            };
                        });
                    });
                    cols[5].vertical_centered(|ui| {
                        if ui
                            .add(egui::Button::new("Save Image").min_size(Vec2::from([
                                ui.available_width() * 0.95,
                                ui.available_height() * 0.5,
                            ])))
                            .clicked()
                        {
                            let res = utils::save_image(app);
                            if res {
                                app.toasts.success("Saved image!").duration(Option::from(Duration::from_secs(1)));
                            } else {
                                app.toasts.error("Could not save the image").duration(Option::from(Duration::from_secs(3)));
                            }
                        }
                    });
                });
            });
        });
}

pub fn central_panel(ctx: &egui::Context, app: &mut ui::app::AsciiApp) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            let (width, height) = (2. * ui.available_width() / 3., ui.available_height());
            ui.allocate_ui_with_layout(
                [width, height].into(),
                egui::Layout::top_down(egui::Align::TOP),
                |ui| {

                    // Check if the downscaled image needs resizing
                    if app.image_config.downscaled_display_img.width() > width as u32 {
                        app.image_config.downscaled_display_img = app.image_config.downscaled_display_img.resize(
                            width as u32,
                            height as u32,
                            image::imageops::FilterType::Nearest
                        );
                    }

                    ui.vertical_centered_justified(|ui| {
                    //     let mut img = &app.image_config.downscaled_display_img;
                    //     let (texture_id, original_size) = {
                    //         let tex = utils::convert_image_to_texture(img, ui).unwrap();
                    //         // the "natural" size of the texture in points:
                    //         let original_size = Vec2::new(img.width() as f32, img.height() as f32);
                    //         (tex, original_size)
                    //     };
                    // 
                    //     // Compute the rectangle we want to use as our 'canvas' for pan/zoom:
                    //     let avail = Vec2::new(ui.available_width() * 0.9, ui.available_height() * 0.9);
                    //     let desired_size = original_size * app.zoom;
                    //     let size = desired_size.min(avail); // optional clamp to available
                    // 
                    //     // Allocate that space with drag-sense:
                    //     let (rect, response) = ui.allocate_exact_size(size, egui::Sense::drag());
                    // 
                    //     // -- PAN: click-drag to move --
                    //     if response.dragged() {
                    //         app.pan += response.drag_delta();
                    //     }
                    // 
                    //     // -- ZOOM: mouse wheel centered at cursor --
                    //     if response.hovered() {
                    //         let scroll = ui.input(|i| i.smooth_scroll_delta.y);
                    //         if scroll.abs() > f32::EPSILON {
                    //             // adjust this factor to taste:
                    //             let zoom_factor = (1.0 + scroll * 0.001).clamp(0.1, 10.0);
                    //             if let Some(mouse_pos) = ui.input(|i| i.pointer.hover_pos()) {
                    //                 // image-space point under the cursor before zoom
                    //                 let image_rel = (mouse_pos - rect.min - app.pan) / app.zoom;
                    //                 // update zoom
                    //                 app.zoom *= zoom_factor;
                    //                 // re-center pan so that the same image point stays under cursor
                    //                 app.pan = mouse_pos - rect.min - image_rel * app.zoom;
                    //             }
                    //         }
                    //     }
                    // 
                    //     // finally, paint the image at rect.min + pan, with size = original_size * zoom
                    //     let image_rect = egui::Rect::from_min_size(rect.min + app.pan, original_size * app.zoom);
                    //     let uv_rect = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)); // Full image UV
                    //     
                    //     ui.painter().image(
                    //         texture_id.id,
                    //         image_rect,
                    //         uv_rect,
                    //         egui::Color32::WHITE
                    //     );
                    //     
                        ui
                            .add_sized(
                                Vec2::new(ui.available_width() * 0.9, ui.available_height() * 0.9),
                                egui::Image::new(ui::utils::convert_image_to_texture(
                                    &app.image_config.downscaled_display_img,
                                    ui,
                                ).unwrap()).shrink_to_fit(),
                            )
                    // 
                    });
                    ui.add_space(ui.available_height() * 0.2);
                    let buttons = ["Original", "Ascii", "Sobel", "Gaussian"];
                    ui.columns(6, |cols| {
                        for (i, &button) in buttons.iter().enumerate() {
                            cols[i + 1].vertical_centered(|ui| {
                                let is_selected = i == app.ui_config.image_type;
                                let button = egui::Button::new(button)
                                    .min_size(Vec2::from([
                                        ui.available_width() * 0.9,
                                        ui.available_height() * 0.9,
                                    ]))
                                    .fill(if is_selected {
                                        egui::Color32::from_rgb(100, 100, 100)
                                    } else {
                                        egui::Color32::from_rgb(60, 60, 60)
                                    });
                                
                                if ui.add(button).clicked() && !is_selected {
                                    app.ui_config.image_type = i;
                                    app.update_images();
                                }
                            });
                        }
                    });
                },
            );

            ui.separator();

            ui.allocate_ui_with_layout(
                [ui.available_width(), ui.available_height()].into(),
                egui::Layout::top_down(egui::Align::TOP),
                |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.heading("Parameter Control");
                        ui.add_space(25.0);
                        let mut sliders_changed = false;

                        sliders_changed |= ui.add(egui::Slider::new(&mut app.parameter_config.sigma_one, 1..=20).text("Sigma One")).drag_stopped();
                        ui.add_space(20.0);
                        sliders_changed |= ui.add(egui::Slider::new(&mut app.parameter_config.sigma_two, 1..=50).text("Sigma Two")).drag_stopped();
                        ui.add_space(20.0);
                        sliders_changed |= ui.add(egui::Slider::new(&mut app.parameter_config.threshold, 1..=50).text("Threshold")).drag_stopped();
                        ui.add_space(20.0);
                        sliders_changed |= ui.add(egui::Slider::new(&mut app.parameter_config.edge_threshold, 1..=10).text("Edge Threshold")).on_hover_text("Determines how many pixels are needed to form an edge").drag_stopped();
                        ui.add_space(20.0);
                        sliders_changed |= ui.add(egui::Slider::new(&mut app.parameter_config.tau, 0.1..=1.0).text("Tau")).drag_stopped();

                        ui.add_space(40.0);
                        sliders_changed |= ui.add(egui::Slider::new(&mut app.parameter_config.gamma, 0.1..=3.0).text("Gamma")).drag_stopped();

                        ui.add_space(40.0);
                        sliders_changed |= ui.add(egui::Slider::new(&mut app.ui_config.down_scale_factor_id, 0..=(app.ui_config.down_scale_factors.len() as u32)-1).text("Scale Down").custom_formatter(|x, _| {
                            format!("{}", app.ui_config.down_scale_factors[x as usize])
                        })).drag_stopped();
                        ui.add_space(20.0);
                        sliders_changed |= ui.add(egui::Slider::new(&mut app.ui_config.up_scale_factor_id, 0..=(app.ui_config.up_scale_factors.len() as u32)-1).text("Upscale").custom_formatter(|x, _| {
                            format!("{}", app.ui_config.up_scale_factors[x as usize])
                        })).drag_stopped();

                        if sliders_changed {
                            app.changed = true;
                            app.update_images();
                        }
                        
                        ui.add_space(ui.available_height()*0.8);
                        if ui.add(egui::Button::new("Apply")
                            .min_size(Vec2::from([ui.available_width(), ui.available_height()*0.9]))
                            .fill(egui::Color32::from_rgb(60, 60, 60))
                            ).on_hover_text("Apply the changes to the image").clicked() {
                                app.changed = true;
                                app.update_images();
                                app.toasts.success("Applied!").duration(Option::from(Duration::from_secs(2)));
                            };
                    });
                },
            );
        });
    });
}