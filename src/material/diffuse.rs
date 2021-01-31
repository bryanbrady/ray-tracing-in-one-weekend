use crate::color::{color, Color};
use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType};
use crate::ray::Ray;
use crate::texture::{Texture, TextureColor};
use crate::vec::Vec3;
use std::sync::Arc;

// Diffuse
#[derive(Debug, Clone)]
pub struct Diffuse {
    pub emit: Texture,
}

impl Diffuse {
    pub fn new(emit: Texture) -> Arc<MaterialType> {
        Arc::new(MaterialType::from(Diffuse { emit: emit }))
    }
}

impl Material for Diffuse {
    fn emitted(&self, _ray: &Ray, hit: &HitRecord, u: f64, v: f64, p: Vec3) -> Color {
        if hit.front_face {
            return self.emit.value(u, v, p);
        } else {
            return color(0.0, 0.0, 0.0);
        }
    }
}
