#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::accesskit::Role::Image;

fn main() -> Result<(), eframe::Error> {
    /*use std::fs::File;
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];
    // Inspect more details of the last read frame.
    let in_animation = reader.info().frame_control.is_some();*/

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
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
    image_sequence: Vec<Vec<u8>>,
    dropped_files: Vec<egui::DroppedFile>,
    picked_path: Option<String>,
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
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Import Images").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            self.picked_path = Some(path.display().to_string());
                            self.image_sequence.push(vec![0]);
                        }
                    }
                    if let Some(picked_path) = &self.picked_path {
                        ui.horizontal(|ui| {
                            ui.label("Picked file:");
                            ui.monospace(picked_path);
                        });
                    }
                });
                ui.menu_button("Edit", |ui| if ui.button("Features TBD").clicked() {})
            });
            ui.heading("Speedy Spritesheets");

            ui.label(format!(
                "Number of images imported: {}",
                self.image_sequence.len()
            ));
            for image in &self.image_sequence {
                ui.label(format!("{:?}", image));
            }
        });
    }
}
