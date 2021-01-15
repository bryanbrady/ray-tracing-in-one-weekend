use crate::color::{Color, color};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::texture::{Texture,TextureColor};
use crate::vec::Vec3;
use enum_dispatch::enum_dispatch;
use rand::prelude::*;
use rand::rngs::SmallRng;

pub struct Scatter {
    pub attenuation: Color,
    pub scattered: Ray
}

#[enum_dispatch]
pub trait Material: Clone {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter>;
}

#[enum_dispatch(Material)]
#[derive(Debug, Clone, Copy)]
pub enum MaterialType {
    Lambertian,
    Metal,
    Dielectric,
}

impl Default for MaterialType {
    fn default() -> MaterialType {
        Lambertian::new(Texture::default())
    }
}

// Lambertian
#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Texture
}

impl Lambertian {
    pub fn new(albedo: Texture) -> MaterialType {
        MaterialType::from( Lambertian { albedo: albedo })
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let scatter_direction = hit.normal + Vec3::random_unit_vector(rng);
        let scatter_direction = if scatter_direction.near_zero() { hit.normal } else { scatter_direction };
        let scattered = Ray { origin: hit.point, direction: scatter_direction, time: ray.time };
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        Some(Scatter {
            scattered: scattered,
            attenuation: attenuation
        })
    }
}

// Metal
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Texture,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Texture, fuzz: f64) -> MaterialType {
        MaterialType::from( Metal {
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 }
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let reflected = Vec3::reflect(ray.direction.unit_vector(), hit.normal);
        let scattered = Ray {
            origin: hit.point,
            direction: reflected + self.fuzz * Vec3::random_in_unit_sphere(rng),
            time: ray.time
        };
        let attenuation = self.albedo.value(hit.u, hit.v, hit.point);
        if scattered.direction.dot(hit.normal) > 0.0 {
            return Some(Scatter {scattered: scattered, attenuation: attenuation})
        }
        return None
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub ir: f64
}

impl Dielectric {
    pub fn new(ir: f64) -> MaterialType {
        MaterialType::from( Dielectric { ir: ir })
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = f64::powi((1.0 - ref_idx) / (1.0 + ref_idx), 2);
        return r0 + (1.0-r0) * f64::powi(1.0-cosine, 5);

    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let refraction_ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };
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
        Some(Scatter{
            scattered: Ray { origin: hit.point, direction: direction, time: ray.time},
            attenuation: color(1.0, 1.0, 1.0)
        })
    }
}
