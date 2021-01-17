use crate::color::{color, Color};
use crate::texture::{perlin::Perlin, Texture, TextureColor};
use crate::vec::Vec3;

// TurbulenceTexture
#[derive(Debug, Clone)]
pub struct TurbulenceTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl TurbulenceTexture {
    pub fn new(seed: u64, scale: f64) -> Texture {
        Texture::from(TurbulenceTexture {
            noise: Perlin::new(seed),
            scale: scale,
        })
    }
}

impl TextureColor for TurbulenceTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Color {
        color(1.0, 1.0, 1.0) * self.noise.turb(p * self.scale, 7)
    }
}
