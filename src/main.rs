#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use eframe::egui;
use eframe::egui::{menu, CollapsingHeader, ScrollArea};

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

#[derive(Default, Debug, Clone, PartialEq)]
struct Image {
    path: PathBuf,
    data: Vec<u8>,
}

impl Image {
    pub fn new(filename: PathBuf) -> Self {
        Self {
            path: filename,
            data: Vec::new(),
        }
    }

    fn import_from_path(&mut self) {}
}

#[derive(Default)]
struct MyApp {
    image_sequence: Vec<Image>,
    selected_image: Option<Image>,
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import Images").clicked() {
                        if let Some(paths) = rfd::FileDialog::new()
                            .add_filter("PNG", &["png"])
                            .pick_files()
                        {
                            for path in paths {
                                self.image_sequence.push(Image::new(path));
                            }
                            for image in &mut self.image_sequence {
                                image.import_from_path();
                            }
                        }
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Speedy Spritesheets");

            ui.label(format!(
                "Number of images imported: {}",
                self.image_sequence.len()
            ));

            // if self.image_sequence.len() > 0 {
            ui.heading("Imported Images");
            ui.horizontal(|ui| {
                ui.label("Show data for image:");
                egui::ComboBox::from_label("")
                    .selected_text(if let Some(selected_image) = &self.selected_image {
                        path_buf_to_string(&selected_image.path)
                    } else if self.image_sequence.len() == 0 {
                        "No imported images".to_string()
                    } else {
                        "Select image".to_string()
                    })
                    .show_ui(ui, |ui| {
                        ui.style_mut().wrap = Some(false);
                        for image in &self.image_sequence {
                            ui.selectable_value(
                                &mut self.selected_image,
                                Some(image.clone()),
                                path_buf_to_string(&image.path),
                            );
                        }
                        ui.style_mut().wrap = Some(true);
                    });
            });
            ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                // ui.monospace(format!("{:?}", self.selected_image));
                if let Some(image) = &self.selected_image {
                    ui.add(egui::Image::from_bytes("bytes://image", image.data.clone()));
                }
            });
            CollapsingHeader::new("Imported Image Paths").show(ui, |ui| {
                ScrollArea::vertical().auto_shrink(true).show(ui, |ui| {
                    for image in &self.image_sequence {
                        ui.monospace(format!("{}", image.path.display().to_string()));
                    }
                });
            });
            // }

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

fn path_buf_to_string(path_buf: &PathBuf) -> String {
    path_buf
        .file_name()
        .expect("selected file terminated in `..` or was empty")
        .to_str()
        .expect("selected filename was invalid unicode")
        .to_string()
}
