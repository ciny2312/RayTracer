use crate::hittable_list::texture::Texture;
use image::{GenericImageView, Pixel, Rgba};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct RtwImage {
    pub width: u32,
    pub height: u32,
    float_pixels: Vec<[f64; 3]>,
}
impl RtwImage {
    pub fn pixel_data(&self, x: u32, y: u32) -> [f64; 3] {
        self.float_pixels[(y * self.width + x) as usize]
    }
}
pub fn load_image_to_float_array<P: AsRef<Path>>(path: P) -> Texture {
    let img = image::open(path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut float_pixels = Vec::with_capacity((width * height) as usize);

    for pixel in img.pixels() {
        let rgba: Rgba<u8> = pixel.2.to_rgba();
        let r = rgba[0] as f64 / 255.0;
        let g = rgba[1] as f64 / 255.0;
        let b = rgba[2] as f64 / 255.0;

        float_pixels.push([r, g, b]);
    }
    Texture::ImageTexture {
        image: RtwImage {
            width,
            height,
            float_pixels,
        },
    }
}
