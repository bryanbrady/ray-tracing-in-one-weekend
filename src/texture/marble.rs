use crate::color::{color, Color};
use crate::texture::{perlin::Perlin, Texture, TextureColor};
use crate::vec::Vec3;

// MarbleTexture
#[derive(Debug, Clone)]
pub struct MarbleTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl MarbleTexture {
    pub fn new(seed: u64, scale: f64) -> Texture {
        Texture::from(MarbleTexture {
            noise: Perlin::new(seed),
            scale: scale,
        })
    }
}

impl TextureColor for MarbleTexture {
    fn value(&self, _u: f64, _v: f64, p: Vec3) -> Color {
        color(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + f64::sin(self.scale * p.z  + 10. * self.noise.turb(p * self.scale, 7)))
    }
}
