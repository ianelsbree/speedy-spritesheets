use crate::path_buf_to_filename_string;
use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle, TextureOptions};
use image as image_lib;
use image_lib::io::Reader as ImageReader;
use image_lib::DynamicImage;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

/// An image as defined by this tool, containing image data and other metadata
#[derive(Default, Clone, PartialEq)]
pub struct Image {
    id: usize,
    name: String,
    path: PathBuf,
    data: DynamicImage,
    texture: Option<TextureHandle>,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let has_texture = &self.texture.is_some();
        f.debug_struct("Image")
            .field("id", &self.id)
            .field("path", &self.path)
            .field("data", &self.data)
            .field("texture", has_texture)
            .field("name", &self.name)
            .finish()
    }
}

#[allow(dead_code)]
impl Image {
    pub fn new(filename: PathBuf) -> Self {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
        let id = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        let data = ImageReader::open(&filename)
            .expect("could not open image")
            .decode()
            .expect("could not decode image");
        let name = path_buf_to_filename_string(&filename);

        Self {
            id,
            name,
            path: filename,
            data,
            texture: None,
        }
    }

    pub fn load_texture(&mut self, ctx: &egui::Context) {
        let image_buffer = self.data.to_rgba8();
        let size = (self.data.width() as usize, self.data.height() as usize);
        let pixels = image_buffer.into_vec();
        assert_eq!(size.0 * size.1 * 4, pixels.len());
        let pixels = ColorImage::from_rgba_unmultiplied([size.0, size.1], &pixels);
        self.texture = Some(ctx.load_texture(&self.name, pixels, TextureOptions::default()));
    }

    pub fn id(&self) -> usize {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    pub fn data(&self) -> &DynamicImage {
        &self.data
    }
    pub fn texture(&self) -> &Option<TextureHandle> {
        &self.texture
    }
}
