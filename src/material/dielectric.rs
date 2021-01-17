use crate::color::color;
use crate::hittable::HitRecord;
use crate::material::{Material, MaterialType, Scatter};
use crate::ray::Ray;
use crate::vec::Vec3;
use rand::prelude::*;
use rand::rngs::SmallRng;

#[derive(Debug, Clone)]
pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> MaterialType {
        MaterialType::from(Dielectric { ir: ir })
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = f64::powi((1.0 - ref_idx) / (1.0 + ref_idx), 2);
        return r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray.direction.unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(hit.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0
            || Dielectric::reflectance(cos_theta, refraction_ratio) > rng.gen();
        let direction = if cannot_refract {
            Vec3::reflect(unit_direction, hit.normal)
        } else {
            Vec3::refract(unit_direction, hit.normal, refraction_ratio)
        };
        Some(Scatter {
            scattered: Ray {
                origin: hit.point,
                direction: direction,
                time: ray.time,
            },
            attenuation: color(1.0, 1.0, 1.0),
        })
    }
}
