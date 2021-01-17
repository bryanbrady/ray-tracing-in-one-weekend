use crate::color::Color;
use crate::texture::{solidcolor::SolidColor, Texture, TextureColor};
use crate::vec::Vec3;

// CheckerTexture
#[derive(Debug, Clone)]
pub struct CheckerTexture {
    pub odd: Box<Texture>,
    pub even: Box<Texture>,
}

impl CheckerTexture {
    pub fn new(c1: Color, c2: Color) -> Texture {
        Texture::from(CheckerTexture {
            odd: Box::new(SolidColor::new(c1.r, c1.g, c1.b)),
            even: Box::new(SolidColor::new(c2.r, c2.g, c2.b)),
        })
    }
}

impl Default for CheckerTexture {
    fn default() -> CheckerTexture {
        CheckerTexture {
            odd: Box::new(SolidColor::new(0.0, 0.0, 0.0)),
            even: Box::new(SolidColor::new(1.1, 1.1, 1.1)),
        }
    }
}

impl TextureColor for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0 * p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
