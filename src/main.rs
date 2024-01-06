#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use eframe::egui;
use eframe::egui::{menu, CollapsingHeader, ScrollArea};

use image::Image;

mod image;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 400.0])
            .with_drag_and_drop(true),
        ..Default::default()
    };
    eframe::run_native(
        "Speedy Spritesheets",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<MyApp>::default()
        }),
    )
}

#[derive(Default)]
struct MyApp {
    image_sequence: Vec<Image>,
    selected_image: Option<Image>,
}

impl MyApp {
    fn import_images(&mut self) {
        if let Some(paths) = rfd::FileDialog::new()
            .add_filter("PNG", &["png"])
            .pick_files()
        {
            for path in paths {
                self.image_sequence.push(Image::new(path));
            }
        }
    }
    fn no_images_loaded(&self) -> bool {
        self.image_sequence.len() == 0
    }
}

/*impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            image_sequence: Vec::new(),
        }
    }
}*/

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // system menu
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import Images").clicked() {
                        self.import_images();
                        ui.close_menu();
                    }
                    if ui.button("Unload All Images").clicked() {
                        self.image_sequence.clear();
                        ui.close_menu();
                    }
                    /*if ui.button("Close Program").clicked() {
                        ctx.send_viewport_cmd(viewport::ViewportCommand::Close);
                        ui.close_menu();
                    }*/
                });
                ui.menu_button("Edit", |ui| {
                    if ui.button("Features TBD").clicked() {
                        ui.close_menu();
                    }
                })
            });
        });

        // main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                ui.heading("Speedy Spritesheets");

                ui.label(format!(
                    "Number of images imported: {}",
                    self.image_sequence.len()
                ));

                ui.heading("Frames");
                if self.no_images_loaded() {
                    ui.horizontal(|ui| {
                        ui.label("No images loaded");
                        if ui.button("Import images").clicked() {
                            self.import_images();
                        }
                    });
                }
                ui.horizontal(|ui| {
                    ui.label("Show data for image:");
                    egui::ComboBox::from_label("")
                        .selected_text(if let Some(selected_image) = &self.selected_image {
                            &selected_image.name()
                        } else if self.no_images_loaded() {
                            "No imported images"
                        } else {
                            "Select image"
                        })
                        .show_ui(ui, |ui| {
                            ui.style_mut().wrap = Some(false);
                            for image in &self.image_sequence {
                                ui.selectable_value(
                                    &mut self.selected_image,
                                    Some(image.clone()),
                                    image.name(),
                                );
                            }
                            ui.style_mut().wrap = Some(true);
                        });
                });
                ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                    // ui.monospace(format!("{:?}", self.selected_image));
                    if let Some(image) = &mut self.selected_image {
                        // ui.add(egui::Image::from_bytes("bytes://image", image.data.clone()));
                        if image.texture().is_none() {
                            image.load_texture(&ctx);
                        }
                        if let Some(texture) = &image.texture() {
                            ui.add(egui::Image::new(texture).max_size([100.0, 100.0].into()));
                        } else {
                            ui.label("Image is loaded but does not have a texture.");
                        }
                    }
                });
                CollapsingHeader::new("Loaded Images").show(ui, |ui| {
                    ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                        for image in &mut self.image_sequence {
                            if image.texture().is_none() {
                                image.load_texture(&ctx);
                            }
                            if let Some(texture) = &image.texture() {
                                ui.add(egui::Image::new(texture).max_size([100.0, 100.0].into()));
                            } else {
                                ui.label("Image is loaded but does not have a texture.");
                            }
                        }
                    });
                });
                CollapsingHeader::new("Imported Image Paths").show(ui, |ui| {
                    ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                        for image in &self.image_sequence {
                            ui.monospace(format!("{}", image.path().display().to_string()));
                        }
                    });
                });

                // file drag-and-drop
                preview_files_being_dropped(ctx);
                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        let dropped_files = i.raw.dropped_files.clone();
                        for file in dropped_files {
                            if let Some(path) = file.path {
                                self.image_sequence.push(Image::new(path));
                            }
                        }
                    }
                });
            });
        });
    }
}

fn preview_files_being_dropped(ctx: &egui::Context) {
    use egui::*;
    use std::fmt::Write as _;

    if !ctx.input(|i| i.raw.hovered_files.is_empty()) {
        let text = ctx.input(|i| {
            let mut text = "Dropping files:\n".to_owned();
            for file in &i.raw.hovered_files {
                if let Some(path) = &file.path {
                    write!(text, "\n{}", path.display()).ok();
                } else if !file.mime.is_empty() {
                    write!(text, "\n{}", file.mime).ok();
                } else {
                    text += "\n???";
                }
            }
            text
        });

        let painter =
            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));

        let screen_rect = ctx.screen_rect();
        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
        painter.text(
            screen_rect.center(),
            Align2::CENTER_CENTER,
            text,
            TextStyle::Heading.resolve(&ctx.style()),
            Color32::WHITE,
        );
    }
}

fn path_buf_to_filename_string(path_buf: &PathBuf) -> String {
    path_buf
        .file_name()
        .expect("selected file terminated in `..` or was empty")
        .to_str()
        .expect("selected filename was invalid unicode")
        .to_string()
}
