use crate::color::Color;
use crate::texture::{
    checker::CheckerTexture, marble::MarbleTexture, noise::NoiseTexture, solidcolor::SolidColor,
    turbulence::TurbulenceTexture,
};
use crate::vec::Vec3;
use enum_dispatch::enum_dispatch;

pub mod checker;
pub mod marble;
pub mod noise;
pub mod perlin;
pub mod solidcolor;
pub mod turbulence;

#[enum_dispatch]
pub trait TextureColor {
    fn value(&self, u: f64, v: f64, p: Vec3) -> Color;
}

#[enum_dispatch(TextureColor)]
#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor,
    CheckerTexture,
    NoiseTexture,
    TurbulenceTexture,
    MarbleTexture,
}

impl Default for Texture {
    fn default() -> Texture {
        Texture::from(SolidColor::default())
    }
}
