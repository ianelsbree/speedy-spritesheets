#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use ::image::{DynamicImage, GenericImage};
use eframe::egui;
use eframe::egui::{menu, vec2, CollapsingHeader, Pos2, ScrollArea};
use pkg_version::*;

use image::Image;

mod image;

const MAJOR: u32 = pkg_version_major!();
const MINOR: u32 = pkg_version_minor!();
const PATCH: u32 = pkg_version_patch!();

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
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
struct MyApp {
    image_sequence: Vec<Image>,
    selected_image: Option<Image>,
    spritesheet: Option<Image>,
    show_about_viewport: bool,
    about_viewport_tl_corner: Pos2,
    about_width: f32,
    about_height: f32,
}

impl MyApp {
    fn import_images(&mut self) {
        if let Some(paths) = rfd::FileDialog::new()
            .add_filter("PNG", &["png"])
            .pick_files()
        {
            for path in paths {
                self.image_sequence.push(Image::from_path(path));
            }
        }
    }
    fn export_image(&self, image: &Image) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("PNG", &["png"])
            .save_file()
        {
            image.data().save(path).expect("error saving image file");
        }
    }

    fn no_images_loaded(&self) -> bool {
        self.image_sequence.len() == 0
    }
    fn some_images_loaded(&self) -> bool {
        self.image_sequence.len() > 0
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
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about_viewport = true;
                        let mut main_viewport_pos = None;
                        ctx.input(|i| main_viewport_pos = i.viewport().outer_rect);
                        if let Some(main_viewport_pos) = main_viewport_pos {
                            let main_tl_corner = main_viewport_pos.min;
                            let main_width = main_viewport_pos.width();
                            let main_height = main_viewport_pos.height();
                            self.about_width = 300.0;
                            self.about_height = 200.0;
                            self.about_viewport_tl_corner = main_tl_corner
                                + vec2(
                                    main_width / 2.0 - self.about_width / 2.0,
                                    main_height / 2.0 - self.about_height / 2.0,
                                );
                            ui.close_menu();
                        }
                    }
                })
            });
        });

        if self.show_about_viewport {
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("about_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("About Speedy Spritesheets")
                    .with_inner_size([self.about_width, self.about_height])
                    .with_resizable(false)
                    .with_minimize_button(false)
                    .with_maximize_button(false)
                    .with_always_on_top()
                    .with_position(self.about_viewport_tl_corner)
                    .with_active(true),
                |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            let version_text =
                                format!("Speedy Spritesheets v{}.{}.{}", MAJOR, MINOR, PATCH);
                            ui.label(version_text);
                            ui.label("Written by Ian Elsbree");
                            ui.hyperlink_to(
                                "GitHub Repository",
                                "https://github.com/ianelsbree/speedy-spritesheets",
                            );
                            ui.hyperlink_to(
                                "crates.io",
                                "https://crates.io/crates/speedy-spritesheets",
                            );
                        });
                    });

                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_about_viewport = false;
                    }
                },
            );
        }

        // main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().auto_shrink(false).show(ui, |ui| {
                ui.heading("Speedy Spritesheets");

                ui.label(format!(
                    "Number of images imported: {}",
                    self.image_sequence.len()
                ));

                ui.heading("Frames");
                if self.some_images_loaded() {
                    if ui.button("Generate Spritesheet!").clicked() {
                        let width = self
                            .image_sequence
                            .iter()
                            .fold(0, |acc, x| acc + x.data().width());
                        let height = self
                            .image_sequence
                            .iter()
                            .max_by(|x, y| x.data().height().cmp(&y.data().height()))
                            .unwrap()
                            .data()
                            .height();
                        let mut x_offset = 0;
                        let mut ss_data = DynamicImage::new_rgb8(width, height);
                        for image in &self.image_sequence {
                            let data = image.data();
                            ss_data
                                .copy_from(data, x_offset, 0)
                                .expect("error in copying image data into spritesheet");
                            x_offset += data.width();
                        }
                        self.spritesheet = Some(Image::from_dynamic("Spritesheet", ss_data));
                    }
                }
                if let Some(spritesheet) = &mut self.spritesheet {
                    if spritesheet.texture().is_none() {
                        spritesheet.load_texture(&ctx);
                    }
                    if let Some(texture) = &spritesheet.texture() {
                        ui.add(egui::Image::new(texture).max_size([500.0, 5000.0].into()));
                    } else {
                        ui.label("Image is loaded but does not have a texture.");
                    }
                }
                if let Some(spritesheet) = &self.spritesheet {
                    if ui.button("Export spritesheet").clicked() {
                        self.export_image(spritesheet);
                    }
                }
                if self.no_images_loaded() {
                    ui.horizontal(|ui| {
                        ui.label("No images loaded");
                        if ui.button("Import images").clicked() {
                            self.import_images();
                        }
                    });
                }
                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Show data for image")
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
                    ui.monospace(format!("{:#?}", self.selected_image));
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
                            if let Some(path) = image.path() {
                                ui.monospace(format!("{}", path.display().to_string()));
                            }
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
                                if path_buf_to_filename_string(&path).ends_with(".png") {
                                    self.image_sequence.push(Image::from_path(path));
                                }
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
