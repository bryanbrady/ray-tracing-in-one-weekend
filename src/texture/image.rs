use crate::color::{color, Color};
use crate::texture::{Texture, TextureColor};
use crate::util::clamp;
use crate::vec::Vec3;
use image::io::Reader as ImageReader;
use std::sync::Arc;

// Image
#[derive(Debug, Clone)]
pub struct ImageTexture {
    pub data: Arc<Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub bytes_per_pixel: u32,
    pub bytes_per_scanline: u32,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Texture {
        let default = Texture::from(ImageTexture {
            data: Arc::new(vec![0, 1, 1]),
            width: 1,
            height: 1,
            bytes_per_pixel: 3,
            bytes_per_scanline: 3,
        });
        let img = ImageReader::open(filename);
        let img = match img {
            Ok(img) => img,
            Err(error) => {
                eprintln!("Problem opening image: {:?}", error);
                return default;
            }
        };
        let img = match img.decode() {
            Ok(img) => img,
            Err(error) => {
                eprintln!("Problem decoding image: {:?}", error);
                return default;
            }
        };
        let rgb8 = img.to_rgb8();
        let bytes: Vec<u8> = rgb8.as_raw().to_vec();
        let asdf = Texture::from(ImageTexture {
            data: Arc::new(bytes),
            width: rgb8.width(),
            height: rgb8.height(),
            bytes_per_pixel: 3,
            bytes_per_scanline: 3 * rgb8.width(),
        });
        asdf
    }
}

impl TextureColor for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: Vec3) -> Color {
        let uu = clamp(u, 0., 1.);
        let vv = 1. - clamp(v, 0., 1.);
        let i = (uu * (self.width as f64)) as u32;
        let j = (vv * (self.height as f64)) as u32;
        let i = if i >= self.width { self.width - 1 } else { i };
        let j = if j >= self.height { self.height - 1 } else { j };
        let color_scale = 1. / 255.;
        let pixel = (j * self.bytes_per_scanline + i * self.bytes_per_pixel) as usize;
        color(
            color_scale * (self.data[pixel] as f64),
            color_scale * (self.data[pixel + 1] as f64),
            color_scale * (self.data[pixel + 2] as f64),
        )
    }
}
