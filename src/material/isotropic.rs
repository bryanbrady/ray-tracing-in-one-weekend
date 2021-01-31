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
    fn scatter(&self, rayin: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let ray = Ray {
            origin: rayin.origin,
            direction: Vec3::random_in_unit_sphere(rng),
            time: rayin.time,
        };

        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some(Scatter {
            ray: ray,
            attenuation: attenuation,
            pdf: None,
        })
    }
}
