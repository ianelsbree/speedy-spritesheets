use std::fmt::{Debug, Formatter};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle, TextureOptions};
use image as image_lib;
use image_lib::io::Reader as ImageReader;
use image_lib::DynamicImage;

use crate::path_buf_to_filename_string;

/// An image as defined by this tool, containing image data and other metadata
#[derive(Default, Clone, PartialEq)]
pub struct Image {
    id: usize,
    name: String,
    path: Option<PathBuf>,
    data: DynamicImage,
    texture: Option<TextureHandle>,
}

impl Debug for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let has_texture = &self.texture.is_some();
        f.debug_struct("Image")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("path", &self.path)
            .field("data", &"data")
            .field("texture", has_texture)
            .finish()
    }
}

#[allow(dead_code)]
impl Image {
    fn get_uid() -> usize {
        static ID_COUNTER: AtomicUsize = AtomicUsize::new(1);
        ID_COUNTER.fetch_add(1, Ordering::Relaxed)
    }
    pub fn from_path(filename: PathBuf) -> Self {
        let id = Self::get_uid();
        let name = path_buf_to_filename_string(&filename);
        let data = ImageReader::open(&filename)
            .expect("could not open image")
            .decode()
            .expect("could not decode image");
        let path = Some(filename);
        let texture = None;
        Self {
            id,
            name,
            path,
            data,
            texture,
        }
    }
    pub fn from_dynamic<N>(name: N, data: DynamicImage) -> Self
    where
        N: Into<String>,
    {
        let id = Self::get_uid();
        let name = name.into();
        let path = None;
        let texture = None;
        Self {
            id,
            name,
            path,
            data,
            texture,
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
    pub fn path(&self) -> Option<&PathBuf> {
        if let Some(path) = &self.path {
            Some(&path)
        } else {
            None
        }
    }
    pub fn data(&self) -> &DynamicImage {
        &self.data
    }
    pub fn texture(&self) -> &Option<TextureHandle> {
        &self.texture
    }
}
