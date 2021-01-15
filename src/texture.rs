use enum_dispatch::enum_dispatch;
use crate::color::{color,Color};
use crate::vec::Vec3;

#[enum_dispatch]
pub trait TextureColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

#[enum_dispatch(TextureColor)]
#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor,
    CheckerTexture
}

impl Default for Texture{
    fn default() -> Texture {
        Texture::from(SolidColor::default())
    }
}

// SolidColor
#[derive(Debug, Clone)]
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
            odd:  Box::new(SolidColor::new(0.0,0.0,0.0)),
            even: Box::new(SolidColor::new(1.1,1.1,1.1))
        }
    }
}

impl TextureColor for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color {
        let sines = f64::sin(10.0 * p.x) * f64::sin(10.0* p.y) * f64::sin(10.0 * p.z);
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
