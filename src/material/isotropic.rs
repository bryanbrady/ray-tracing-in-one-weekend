use crate::color::{color, Color};
use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType, Scatter};
use crate::ray::Ray;
use crate::texture::{Texture, TextureColor};
use crate::vec::Vec3;
use rand::rngs::SmallRng;
use std::sync::Arc;

// Isotropic
#[derive(Debug, Clone)]
pub struct Isotropic {
    pub albedo: Texture,
}

impl Isotropic {
    pub fn new(albedo: Texture) -> Arc<MaterialType> {
        Arc::new(MaterialType::from(Isotropic { albedo: albedo }))
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let scattered = Ray {
            origin: ray.origin,
            direction: Vec3::random_in_unit_sphere(rng),
            time: ray.time,
        };

        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some(Scatter {
            scattered: scattered,
            attenuation: attenuation,
        })
    }

    fn emitted(&self, _u: f64, _v: f64, _p: Vec3) -> Color {
        return color(0.0, 0.0, 0.0);
    }
}
