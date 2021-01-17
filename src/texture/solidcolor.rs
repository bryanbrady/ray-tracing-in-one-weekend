use crate::color::{color, Color};
use crate::texture::{Texture, TextureColor};
use crate::vec::Vec3;

// SolidColor
#[derive(Debug, Clone)]
pub struct SolidColor {
    pub color: Color,
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Texture {
        Texture::from(SolidColor {
            color: color(r, g, b),
        })
    }
}

impl Default for SolidColor {
    fn default() -> SolidColor {
        SolidColor {
            color: Color::default(),
        }
    }
}

impl TextureColor for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        return self.color;
    }
}
