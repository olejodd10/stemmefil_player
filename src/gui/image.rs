use std::path::Path;
use eframe::egui::{self, Context};

pub struct Image(egui::TextureHandle);

impl Image {
    pub fn new(name: &str, path: &Path, ctx: &Context, w: u32, h: u32) -> Image {
        // https://docs.rs/egui/0.18.0/egui/struct.ColorImage.html#method.from_rgba_unmultiplied
        let image = image::io::Reader::open(path).unwrap().decode().unwrap();
        let resized_image = image.thumbnail(w,h);
        let size = [resized_image.width() as _, resized_image.height() as _];
        let image_buffer = resized_image.to_rgba8(); // Dette er flaskehalsen
        let pixels = image_buffer.as_flat_samples();
        let color_image = egui::ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        Image(ctx.load_texture(name, color_image))
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.image(&self.0, self.0.size_vec2());
    }
}