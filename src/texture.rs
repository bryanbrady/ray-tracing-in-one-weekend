use enum_dispatch::enum_dispatch;
use crate::color::{color,Color};
use crate::vec::Vec3;

#[enum_dispatch]
pub trait TextureColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

#[enum_dispatch(TextureColor)]
#[derive(Debug, Clone, Copy)]
pub enum Texture {
    SolidColor,
}

impl Default for Texture{
    fn default() -> Texture {
        Texture::from(SolidColor::default())
    }
}

// SolidColor
#[derive(Debug, Clone, Copy)]
pub struct SolidColor {
    pub color: Color
}

impl SolidColor {
    pub fn new(r: f64, g: f64, b: f64) -> Texture {
        Texture::from(SolidColor { color: color(r, g, b) })
    }
}

impl Default for SolidColor {
    fn default() -> SolidColor {
        SolidColor { color: Color::default() }
    }
}

impl TextureColor for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        return self.color
    }
}
