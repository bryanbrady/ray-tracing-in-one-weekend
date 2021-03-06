use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType, Scatter};
use crate::ray::Ray;
use crate::texture::{Texture, TextureColor};
use crate::vec::Vec3;
use rand::rngs::SmallRng;
use std::sync::Arc;

// Metal
#[derive(Debug, Clone)]
pub struct Metal {
    pub albedo: Texture,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Texture, fuzz: f64) -> Arc<MaterialType> {
        Arc::new(MaterialType::from(Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
            time: ray.time,
        };
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        if scattered.direction.dot(hit.normal) > 0.0 {
            return Some(Scatter {
                ray: scattered,
                attenuation: attenuation,
                pdf: None,
            });
        }
        return None;
    }
}
