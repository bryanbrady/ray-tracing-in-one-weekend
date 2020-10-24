use crate::color::{Color, color};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Vec3;
use rand::prelude::*;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray
}

pub trait Material{
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector();
        let scattered = Ray { origin: hit.point, direction: scatter_direction };
        let attenuation = self.albedo;
        Some(Scatter {
            scattered: scattered,
            attenuation: attenuation
        })
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        }
    }
}

impl Material for Metal {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let reflected = Vec3::reflect(_ray.direction.unit_vector(), hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere()
        };
        let attenuation = self.albedo;
        if scattered.direction.dot(hit.normal) > 0.0 {
            return Some(Scatter {scattered: scattered, attenuation: attenuation})
        }
        return None
    }
}

pub struct Dielectric {
    pub ir: f64
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = f64::powi((1.0 - ref_idx) / (1.0 + ref_idx), 2);
        return r0 + (1.0-r0) * f64::powi(1.0-cosine, 5);

    }
}

impl Material for Dielectric {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = _ray.direction.unit_vector();
        let cos_theta = f64::min(-unit_direction.dot(hit.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0
                          || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen();
        let direction = if cannot_refract {
            Vec3::reflect(unit_direction, hit.normal)
        } else {
            Vec3::refract(unit_direction, hit.normal, refraction_ratio)
        };
        Some(Scatter{
            scattered: Ray { origin: hit.point, direction: direction},
            attenuation: color(1.0, 1.0, 1.0)
        })
    }
}
