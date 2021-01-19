use crate::color::Color;
use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType, Scatter};
use crate::ray::Ray;
use crate::texture::{Texture, TextureColor};
use crate::vec::Vec3;
use rand::rngs::SmallRng;
use std::sync::Arc;

// Diffuse
#[derive(Debug, Clone)]
pub struct Diffuse {
    pub emit: Texture,
}

impl Diffuse{
    pub fn new(emit: Texture) -> Arc<MaterialType> {
        Arc::new(MaterialType::from(Diffuse{
            emit: emit,
        }))
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord, _rng: &mut SmallRng) -> Option<Scatter> {
        return None;
    }

    fn emitted(&self, u: f64, v: f64, p: Vec3) -> Color {
        return self.emit.value(u, v, p);
    }
}
