use crate::color::{color, Color};
use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType, Scatter};
use crate::ray::Ray;
use crate::texture::{Texture, TextureColor};
use crate::vec::Vec3;
use rand::rngs::SmallRng;
use std::sync::Arc;

// Lambertian
#[derive(Debug, Clone)]
pub struct Lambertian {
    pub albedo: Texture,
}

impl Lambertian {
    pub fn new(albedo: Texture) -> Arc<MaterialType> {
        //eprintln!("Lambertian::new(albedo: {:?}", albedo);
        Arc::new(MaterialType::from(Lambertian { albedo: albedo }))
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector(rng);
        let scatter_direction = if scatter_direction.near_zero() {
            hit.normal
        } else {
            scatter_direction
        };
        let scattered = Ray {
            origin: hit.point,
            direction: scatter_direction,
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
