use crate::color::{color, Color};
use crate::texture::{perlin::Perlin, Texture, TextureColor};
use crate::vec::Vec3;

// NoiseTexture
#[derive(Debug, Clone)]
pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(seed: u64, scale: f64) -> Texture {
        Texture::from(NoiseTexture {
            noise: Perlin::new(seed),
            scale: scale,
        })
    }
}

impl TextureColor for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Color {
        color(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.noise.noise(p * self.scale))
    }
}
